
function addBodyUploadEventListener() {
  const uploader = document.getElementById('body-upload');
  const uploadButton = document.getElementById('body-submit');

  let selectedFile = null;

  uploadButton.addEventListener('click', async event => {
    event.preventDefault();
    selectedFile = uploader.files[0];
    if (!selectedFile) {
    // If no file is selected, download Bushido from the default link
    selectedFile = await fetch('https://huijzer.xyz/files/3f72e1a95933453d.pdf')
      .then(res => res.blob());
    }

    const encoded = CBOR.encode({file: selectedFile});

    console.log(selectedFile);
  });
}

function addEventListeners() {
  addBodyUploadEventListener();
}

addEventListeners();
