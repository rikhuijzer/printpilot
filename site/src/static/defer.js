
function addBodyUploadEventListener() {
  const uploader = document.getElementById('body-upload');
  const uploadButton = document.getElementById('body-submit');

  let selectedFile = null;

  uploadButton.addEventListener('click', event => {
    event.preventDefault();
    selectedFile = uploader.files[0];
    if (!selectedFile) return;

    const encoded = CBOR.encode(selectedFile);

    console.log(selectedFile);
  });
}

function addEventListeners() {
  addBodyUploadEventListener();
}

addEventListeners();
