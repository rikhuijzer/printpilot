function resetBodyUpload() {
  const uploader = document.getElementById('body-upload');
  uploader.value = '';
}

async function submitBodyUpload() {
    let selectedFile = null;
    const uploader = document.getElementById('body-upload');
    const uploadButton = document.getElementById('body-submit');
    selectedFile = uploader.files[0];
    if (!selectedFile) {
        const content = await fetch('/bushido.pdf')
            .then(res => res.blob());
        selectedFile = new File([content], 'bushido.pdf');
    }

    console.log(selectedFile);
    const rawFile = await selectedFile.arrayBuffer();
    const encoder = new CBOR.Encoder();
    const encoded = encoder.encode([{ 
        file: {
            name: selectedFile.name,
            typ: selectedFile.type,
            size: selectedFile.size,
            data: new Uint8Array(rawFile)
        }
    }]);

    const exports = core.instance.exports;
    const ptr = exports.alloc();
    writeToPtr(core.instance, ptr, encoded);
    exports.book_body(ptr);
    const result = readFromPtr(core.instance, ptr);
    exports.dealloc(ptr);
    console.log(`Received: ${result}`);
}
