// Using PDFLib since lopdf works, but is extremely low-level (unwieldy).

function loadPdf() {
  // Get the input element
  const input = document.getElementById("body-upload");
  if (!input || input.type !== "file") {
    console.error("Element is not an HtmlInputElement");
    return;
  }
  const files = input.files;
  let filePromise;
  let name;
  if (!files || files.length === 0) {
    // No file selected, fetch default
    filePromise = fetch("/static/bushido.pdf")
      .then(async resp => {
        const bodyOutput = document.getElementById("body-output");
        if (bodyOutput) {
          bodyOutput.innerHTML = `File selected: ${resp.status}`;
        }
        if (resp.status !== 200) {
          console.log("Failed to fetch bushido.pdf");
          throw new Error("Failed to fetch bushido.pdf");
        }
        const blob = await resp.blob();
        name = "bushido.pdf";
        return blob.arrayBuffer();
      });
  } else {
    const file = files[0];
    name = file.name;
    filePromise = file.arrayBuffer();
  }
  return filePromise.then(arrayBuffer => {
    return {
      name: name,
      data: new Uint8Array(arrayBuffer)
    };
  });
}

async function addJoinedPage(src, dst, left_index, right_index) {
  const height = PDFLib.PageSizes.A4[0];
  const width = PDFLib.PageSizes.A4[1];
  const page = dst.addPage([width, height]);
  const [left, right] = await dst.embedPdf(src, [left_index, right_index]);
  page.drawPage(right, { x: width / 2, y: 0 });
  page.drawPage(left, { x: 0, y: 0 });
}

function ceildiv(a, b) {
  return Math.ceil(a / b);
}

async function createPdf() {
  let pdf = await loadPdf();
  console.log(pdf);

  // Default export is a4 paper, portrait, using millimeters for units
  const doc = await PDFLib.PDFDocument.load(pdf.data);
  const out = await PDFLib.PDFDocument.create();

  const [page] = await out.copyPages(doc, [0]);
  out.addPage(page);

  await addJoinedPage(doc, out, 1, 2);
  await addJoinedPage(doc, out, 3, 5);

  const pdfBytes = await out.save();

  // let name = pdf.name;

  const uint8Array = new Uint8Array(pdfBytes);
  const blob = new Blob([uint8Array], { type: "application/pdf" });
  const objectUrl = URL.createObjectURL(blob);

  const bodyOutput = document.getElementById("body-output");
  if (bodyOutput) {
    bodyOutput.innerHTML = `<a href="${objectUrl}" target="_blank">Open PDF</a>`;
  }
}
