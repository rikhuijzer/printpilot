function writeToPtr(instance, ptr, bytes, bufferLength) {
    const buffer = instance.exports.memory.buffer;
    const view = new Uint8Array(buffer, ptr, bufferLength);
    view.set(bytes.slice(0, bufferLength));
}

function readFromPtr(instance, ptr, bufferLength) {
    const buffer = instance.exports.memory.buffer;
    const bytes = new Uint8Array(buffer, ptr, bufferLength);
    return bytes;
}

function resetBodyUpload() {
  const uploader = document.getElementById('body-upload');
  uploader.value = '';
}

async function submitBodyUpload() {
    let selectedFile = null;
    const uploader = document.getElementById('body-upload');
    const uploadButton = document.getElementById('body-submit');
    selectedFile = uploader.files[0];
    if (selectedFile) {
        if (536870912 < selectedFile.size) {
            alert('File size is too large. Please select a file smaller than 512MB.');
            return;
        }
    } else {
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
    const ptr = exports.alloc(encoded.length);
    writeToPtr(core.instance, ptr, encoded, encoded.length);
    exports.book_body(ptr);
    const result = readFromPtr(core.instance, ptr, encoded.length);
    console.log(`Received: ${result}`);
    exports.dealloc(ptr, encoded.length);
}
