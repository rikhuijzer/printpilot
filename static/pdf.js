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
  const n = src.getPages().length;
  if (n <= left_index && n <= right_index) {
    return;
  }
  const height = PDFLib.PageSizes.A4[0];
  const width = PDFLib.PageSizes.A4[1];
  const page = dst.addPage([width, height]);
  if (left_index < n && right_index < n) {
    const [left, right] = await dst.embedPdf(src, [left_index, right_index]);
    page.drawPage(right, { x: width / 2, y: 0 });
    page.drawPage(left, { x: 0, y: 0 });
  } else if (left_index < n) {
    const [left] = await dst.embedPdf(src, [left_index]);
    page.drawPage(left, { x: 0, y: 0 });
  } else if (right_index < n) {
    const [right] = await dst.embedPdf(src, [right_index]);
    page.drawPage(right, { x: width / 2, y: 0 });
  }
}

function ceildiv(a, b) {
  return Math.ceil(a / b);
}

async function setPdfLink(doc, id, name) {
  let pdfBytes = await doc.save();
  const uint8Array = new Uint8Array(pdfBytes);
  const blob = new Blob([uint8Array], { type: "application/pdf" });
  const objectUrl = URL.createObjectURL(blob);

  const bodyOutput = document.getElementById(id);
  if (bodyOutput) {
    bodyOutput.innerHTML = `<button class='body-output' onclick="window.open('${objectUrl}', '_blank')">Open ${name}</button>`;
  }
}

async function createPdf() {
  let pdf = await loadPdf();
  console.log(pdf);

  // Default export is a4 paper, portrait, using millimeters for units
  const doc = await PDFLib.PDFDocument.load(pdf.data);
  const even = await PDFLib.PDFDocument.create();
  const odd = await PDFLib.PDFDocument.create();

  const n = doc.getPages().length;
  console.log(`n: ${n}`);
  let half = ceildiv(n, 2);
  // If n is odd, we need to add one since we are printing duplex.
  // Can lead to 3 empty pages in the worst case.
  half = n % 2 == 0 ? half : half + 1;
  console.log(`half: ${half}`);
  for (let i = 0; i < ceildiv(half, 2); i++) {
    console.log(`i: ${i}`);
    // half + 2 : 2 : n
    const left_index = (half + 2) + (2 * i);
    console.log(`left_index: ${left_index}`);
    // 2 : 2 : half
    const right_index = (1 + (2 * i));
    console.log(`right_index: ${right_index}`);
    // Flip since we're printing on the back of the odd pages.
    await addJoinedPage(doc, even, right_index, left_index);
  }

  for (let i = 0; i < ceildiv(half, 2); i++) {
    console.log(`i: ${i}`);
    // half + 1 : 2 : n
    const left_index = (half + 1) + (2 * i);
    console.log(`left_index: ${left_index}`);
    const right_index = 2 * i;
    console.log(`right_index: ${right_index}`);
    await addJoinedPage(doc, odd, left_index, right_index);
  }

  const evenBytes = await even.save();
  const oddBytes = await odd.save();

  await setPdfLink(even, "body-output-even", "even pages");
  await setPdfLink(odd, "body-output-odd", "odd pages");
  // await setPdfLink(doc, "body-output-all", "all pages");
  // let name = pdf.name;

}
