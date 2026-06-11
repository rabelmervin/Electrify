wit_bindgen::generate!({
    world: "invoice-renderer",
    path: "../wit",
});

use exports::demo::invoice::renderer::Guest;
use demo::invoice::types::InvoiceData;

struct PdfRenderer;

impl Guest for PdfRenderer {
    fn render_pdf(data: InvoiceData) -> Vec<u8> {
        // Mocking a PDF generation by creating a formatted text payload.
        // In a real application, you would use a PDF library here.
        let mut pdf_content = String::new();
        pdf_content.push_str("%PDF-1.4\n");
        pdf_content.push_str("1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
        pdf_content.push_str("=======================================\n");
        pdf_content.push_str("              INVOICE                  \n");
        pdf_content.push_str("=======================================\n");
        pdf_content.push_str(&format!("Customer: {}\n", data.customer_name));
        pdf_content.push_str("---------------------------------------\n");
        pdf_content.push_str(&format!("Subtotal: ${:.2}\n", data.subtotal));
        pdf_content.push_str(&format!("Discount: -${:.2}\n", data.discount));
        pdf_content.push_str(&format!("Tax:      +${:.2}\n", data.tax));
        pdf_content.push_str("---------------------------------------\n");
        pdf_content.push_str(&format!("TOTAL:    ${:.2}\n", data.total));
        pdf_content.push_str("=======================================\n");
        pdf_content.push_str("%%EOF\n");

        pdf_content.into_bytes()
    }
}

export!(PdfRenderer);
