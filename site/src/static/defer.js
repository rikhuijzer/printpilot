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
    const encoder = new CBOR.Encoder();
    const encoded = encoder.encode([{file: 'a'}]);
}

function addEventListeners() {
  addBodyUploadEventListener();
}

addEventListeners();
