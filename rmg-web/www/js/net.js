"use strict";
// Function to download an HTTP link to a blob
function downloadHttpLinkToBlob(link) {
    fetch(link)
        .then((response) => response.blob())
        .then((blob) => {
        // Do something with the blob, for example save it or use it further
        console.debug("Downloaded Blob");
        return blob;
    })
        .catch((error) => {
        // Handle any errors during the fetch
        console.error("Error downloading HTTP link:", error);
    });
    return null;
}
// Function to save the content of an HTML input to a blob
function saveHtmlInputToBlob(inputFile) {
    let file = inputFile.files?.[0];
    if (file) {
        let reader = new FileReader();
        reader.onload = (event) => {
            // Do something with the loaded content, for example save it or use it
            // further
            let content = event.target?.result;
            let blob = new Blob([content], { type: "text/html" });
            // console.debug("Saved Blob");
            return blob;
        };
        reader.readAsText(file);
    }
    return null;
}
