wit_bindgen::generate!({
    world: "order-processor",
    path: "../wit",
    generate_all,
});

use demo::invoice::renderer::render_pdf;
use demo::invoice::types::{InvoiceData, OrderItem};
use exports::wasi::http::incoming_handler::Guest as HttpGuest;
use wasi::http::types::{
    Fields, IncomingBody, IncomingRequest, Method, OutgoingBody, OutgoingResponse,
    ResponseOutparam,
};
use wasi::io::streams::StreamError;

const UI_HTML: &str = include_str!("../../ui/index.html");

struct OrderProcessor;

fn compute_invoice(input: Order) -> Result<Vec<u8>, String> {
    // 1. Validate input
    if input.customer_name.trim().is_empty() {
        return Err("Customer name cannot be empty".to_string());
    }
    if input.items.is_empty() {
        return Err("Order must contain at least one item".to_string());
    }
    if input.discount_percent < 0.0 || input.discount_percent > 100.0 {
        return Err("Invalid discount percentage".to_string());
    }

    // 2. Calculate subtotal
    let mut subtotal = 0.0;
    for item in &input.items {
        if item.price < 0.0 {
            return Err(format!("Invalid price for item {}", item.name));
        }
        subtotal += item.price * (item.quantity as f32);
    }

    // 3. Apply discount
    let discount = subtotal * (input.discount_percent / 100.0);
    let after_discount = subtotal - discount;

    // 4. Calculate tax (e.g. flat 8% rate)
    let tax_rate = 0.08;
    let tax = after_discount * tax_rate;

    // 5. Final Total
    let total = after_discount + tax;

    let invoice = InvoiceData {
        customer_name: input.customer_name,
        subtotal,
        tax,
        discount,
        total,
    };

    // 6. Call Component Two (PDF Renderer) via WIT interface
    let pdf_bytes = render_pdf(&invoice);

    Ok(pdf_bytes)
}

impl Guest for OrderProcessor {
    fn process_order(input: Order) -> Result<Vec<u8>, String> {
        compute_invoice(input)
    }
}

impl HttpGuest for OrderProcessor {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let method = request.method();
        let path = request.path_with_query().unwrap_or_else(|| "/".to_string());
        let route = path.split('?').next().unwrap_or("/");

        match (&method, route) {
            (Method::Get, "/") | (Method::Get, "/index.html") => {
                respond(response_out, 200, "text/html; charset=utf-8", UI_HTML.as_bytes());
            }
            (Method::Post, "/order") => {
                // Capture the content type before consuming the request body.
                let content_type = header_value(&request, "content-type").unwrap_or_default();
                let body = match read_body(request) {
                    Ok(b) => b,
                    Err(e) => {
                        return respond(
                            response_out,
                            400,
                            "text/plain; charset=utf-8",
                            format!("failed to read request body: {e}").as_bytes(),
                        );
                    }
                };
                let order = if content_type.contains("json") {
                    parse_json_order(&body)
                } else {
                    parse_form_order(&body)
                };
                match order.and_then(compute_invoice) {
                    // The renderer's "PDF" is a mocked text payload, so serve it
                    // as plain text to stay readable in browsers and curl.
                    Ok(invoice) => {
                        respond(response_out, 200, "text/plain; charset=utf-8", &invoice)
                    }
                    Err(e) => respond(response_out, 400, "text/plain; charset=utf-8", e.as_bytes()),
                }
            }
            _ => respond(response_out, 404, "text/plain; charset=utf-8", b"not found"),
        }
    }
}

fn header_value(request: &IncomingRequest, name: &str) -> Option<String> {
    let values = request.headers().get(&name.to_string());
    values
        .first()
        .map(|v| String::from_utf8_lossy(v).to_string())
}

fn read_body(request: IncomingRequest) -> Result<Vec<u8>, String> {
    let body: IncomingBody = request
        .consume()
        .map_err(|()| "request body already consumed".to_string())?;
    let mut data = Vec::new();
    {
        let stream = body
            .stream()
            .map_err(|()| "failed to open request body stream".to_string())?;
        loop {
            match stream.blocking_read(64 * 1024) {
                Ok(chunk) => data.extend_from_slice(&chunk),
                Err(StreamError::Closed) => break,
                Err(StreamError::LastOperationFailed(e)) => {
                    return Err(format!("stream error: {}", e.to_debug_string()));
                }
            }
        }
    }
    IncomingBody::finish(body);
    Ok(data)
}

fn respond(out: ResponseOutparam, status: u16, content_type: &str, body: &[u8]) {
    let headers = Fields::new();
    let _ = headers.set(&"content-type".to_string(), &[content_type.as_bytes().to_vec()]);
    let response = OutgoingResponse::new(headers);
    let _ = response.set_status_code(status);
    let outgoing_body = response.body().expect("outgoing body");
    ResponseOutparam::set(out, Ok(response));
    {
        let stream = outgoing_body.write().expect("outgoing body stream");
        for chunk in body.chunks(4096) {
            if stream.blocking_write_and_flush(chunk).is_err() {
                // Client disconnected; nothing more to send.
                return;
            }
        }
    }
    let _ = OutgoingBody::finish(outgoing_body, None);
}

fn parse_json_order(body: &[u8]) -> Result<Order, String> {
    let v: serde_json::Value =
        serde_json::from_slice(body).map_err(|e| format!("invalid JSON: {e}"))?;
    let customer_name = v
        .get("customer-name")
        .or_else(|| v.get("customer_name"))
        .and_then(|c| c.as_str())
        .ok_or_else(|| "missing customer-name".to_string())?
        .to_string();
    let items = v
        .get("items")
        .and_then(|i| i.as_array())
        .ok_or_else(|| "missing items".to_string())?
        .iter()
        .map(|it| OrderItem {
            name: it
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or_default()
                .to_string(),
            price: it.get("price").and_then(|p| p.as_f64()).unwrap_or(0.0) as f32,
            quantity: it.get("quantity").and_then(|q| q.as_u64()).unwrap_or(0) as u32,
        })
        .collect();
    let discount_percent = v
        .get("discount-percent")
        .or_else(|| v.get("discount_percent"))
        .and_then(|d| d.as_f64())
        .unwrap_or(0.0) as f32;
    Ok(Order {
        customer_name,
        items,
        discount_percent,
    })
}

fn parse_form_order(body: &[u8]) -> Result<Order, String> {
    let pairs: Vec<(String, String)> = form_urlencoded::parse(body).into_owned().collect();
    let field = |key: &str| {
        pairs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    };

    let customer_name = field("customer").ok_or_else(|| "missing customer".to_string())?;
    let discount_percent = field("discount")
        .and_then(|d| d.parse::<f32>().ok())
        .unwrap_or(0.0);

    let mut items = Vec::new();
    for n in 1.. {
        let Some(name) = field(&format!("item_name_{n}")) else {
            break;
        };
        let quantity = field(&format!("item_qty_{n}"))
            .and_then(|q| q.parse::<u32>().ok())
            .unwrap_or(0);
        let price = field(&format!("item_price_{n}"))
            .and_then(|p| p.parse::<f32>().ok())
            .unwrap_or(0.0);
        if !name.trim().is_empty() && quantity > 0 {
            items.push(OrderItem {
                name,
                price,
                quantity,
            });
        }
    }

    Ok(Order {
        customer_name,
        items,
        discount_percent,
    })
}

export!(OrderProcessor);
