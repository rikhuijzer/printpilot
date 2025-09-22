// Using PDFLib since lopdf works, but is extremely low-level (unwieldy).

async function createPdf() {
  const pdfDoc = await PDFLib.PDFDocument.create()
  const timesRomanFont = await pdfDoc.embedFont(PDFLib.StandardFonts.TimesRoman)

  const page = pdfDoc.addPage()
  const { width, height } = page.getSize()
  const fontSize = 30
  page.drawText('Creating PDFs in JavaScript is awesome!', {
    x: 50,
    y: height - 4 * fontSize,
    size: fontSize,
    font: timesRomanFont,
    color: PDFLib.rgb(0, 0.53, 0.71),
  })

  const pdfBytes = await pdfDoc.save();
  const uint8Array = new Uint8Array(pdfBytes);
  const blob = new Blob([uint8Array], { type: "application/pdf" });
  const objectUrl = URL.createObjectURL(blob);

  const bodyOutput = document.getElementById("body-output");
  if (bodyOutput) {
    bodyOutput.innerHTML = `<a href="${objectUrl}" target="_blank">Open PDF</a>`;
  }
}
