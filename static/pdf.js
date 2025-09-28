// Using PDFLib since lopdf works, but is extremely low-level (unwieldy).
// I also couldn't get Typst to compile.

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

const A4Width = PDFLib.PageSizes.A4[0];
const A4Height = PDFLib.PageSizes.A4[1];

async function addJoinedPage(src, dst, left_index, right_index) {
  const n = src.getPages().length;
  if (n <= left_index && n <= right_index) {
    return;
  }
  const page = dst.addPage([A4Height, A4Width]);
  if (left_index < n && right_index < n) {
    const [left, right] = await dst.embedPdf(src, [left_index, right_index]);
    page.drawPage(right, { x: A4Height / 2, y: 0 });
    page.drawPage(left, { x: 0, y: 0 });
  } else if (left_index < n) {
    const [left] = await dst.embedPdf(src, [left_index]);
    page.drawPage(left, { x: 0, y: 0 });
  } else if (right_index < n) {
    const [right] = await dst.embedPdf(src, [right_index]);
    page.drawPage(right, { x: A4Height / 2, y: 0 });
  }
}

function ceildiv(a, b) {
  return Math.ceil(a / b);
}

function resetPdfLink(id) {
  const bodyOutput = document.getElementById(id);
  if (bodyOutput) {
    bodyOutput.innerHTML = "<div></div>";
  }
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
  resetPdfLink("body-output-name");
  resetPdfLink("body-output-even");
  resetPdfLink("body-output-odd");
  resetPdfLink("body-output-all");
  const fieldsets = document.getElementsByClassName("body-output-fieldset");
  for (const fieldset of fieldsets) {
    if (fieldset) {
      fieldset.style.display = "none";
    }
  }

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
  const evenBytes = await even.save();
  const outputName = document.getElementById("body-output-name");
  if (outputName) {
    outputName.innerHTML = `${pdf.name}:`;
  }
  for (const fieldset of fieldsets) {
      if (fieldset) {
      fieldset.style.display = "flex";
    }
  }
  await setPdfLink(even, "body-output-even", "EVEN pages");

  for (let i = 0; i < ceildiv(half, 2); i++) {
    console.log(`i: ${i}`);
    // half + 1 : 2 : n
    const left_index = (half + 1) + (2 * i);
    console.log(`left_index: ${left_index}`);
    const right_index = 2 * i;
    console.log(`right_index: ${right_index}`);
    await addJoinedPage(doc, odd, left_index, right_index);
  }
  const oddBytes = await odd.save();
  await setPdfLink(odd, "body-output-odd", "ODD pages");

  const all = await PDFLib.PDFDocument.create();
  // Merge odd and even PDFs.
  const oddPages = await odd.getPages();
  const evenPages = await even.getPages();
  const totalPages = oddPages.length + evenPages.length;

  let oddIdx = 0;
  let evenIdx = 0;
  for (let i = 0; i < totalPages; i++) {
    if (i % 2 === 0 && oddIdx < oddPages.length) {
      const [oddPage] = await all.copyPages(odd, [oddIdx]);
      all.addPage(oddPage);
      oddIdx++;
    } else if (i % 2 === 1 && evenIdx < evenPages.length) {
      const [evenPage] = await all.copyPages(even, [evenIdx]);
      all.addPage(evenPage);
      evenIdx++;
    }
  }

  await setPdfLink(all, "body-output-all", "ALL pages");
}

function mmToPt(distance) {
  // A4 height: 297 mm should be 842 pt.
  // A4 width: 210 mm should be 595 pt.
  const inchToMm = 25.4;
  return distance * 72 / inchToMm;
}

function drawBackLine(page, font, loc, inner) {
  const opacity = 0.5;
  if (inner) {
    page.drawLine({
      start: { x: loc, y: 0 },
      end: { x: loc, y: A4Width },
      thickness: 1,
      dashArray: [],
      opacity,
    });
  } else {
    const length = 6;
    page.drawLine({
      start: { x: loc, y: 2 * length },
      end: { x: loc, y: 3 * length },
      thickness: 1,
      dashArray: [],
      opacity,
    });
    page.drawLine({
      start: { x: loc, y: A4Width - 2 * length },
      end: { x: loc, y: A4Width - 3 * length },
      thickness: 1,
      dashArray: [],
      opacity,
    });
  }
}

async function createCover() {
  const doc = await PDFLib.PDFDocument.create();
  // doc.registerFontkit(fontkit);

  // const fontBytes = await fetch('/static/EBGaramond-Italic-VariableFont_wght.ttf').then(res => res.arrayBuffer());
  const font = await doc.embedFont(PDFLib.StandardFonts.TimesRoman);

  const front = doc.addPage([A4Height, A4Width]);
  const titleElem = document.getElementById('cover-title');
  let text = titleElem.value;
  const fontSize = 20;
  const textWidth = font.widthOfTextAtSize(text, fontSize);
  const textHeight = font.heightAtSize(fontSize);
  front.drawText(text, {
    x: A4Height / 2 + textHeight / 2,
    y: A4Width / 2 - textHeight / 2,
    font,
    size: fontSize,
    rotate: PDFLib.degrees(90),
  });

  const back = doc.addPage([A4Height, A4Width]);
  let spineWidthElem = document.getElementById('spine-width');
  let spineWidth = spineWidthElem.value;
  const middle = A4Height / 2;
  console.log(`spineWidth: ${spineWidth} mm`);
  spineWidth = mmToPt(spineWidth);
  console.log(`spineWidth: ${spineWidth} pt`);
  drawBackLine(back, font, middle + spineWidth / 2, true);
  drawBackLine(back, font, middle - spineWidth / 2, true);

  await setPdfLink(doc, "cover-output", "Cover");
}
