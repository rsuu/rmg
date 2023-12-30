// Function to download an HTTP link to a blob
function downloadHttpLinkToBlob(link: string): Blob | null {
  fetch(link)
    .then((response: Response) => response.blob())
    .then((blob: Blob) => {
      // Do something with the blob, for example save it or use it further
      console.debug("Downloaded Blob");

      return blob;
    })
    .catch((error: Error) => {
      // Handle any errors during the fetch
      console.error("Error downloading HTTP link:", error);
    });

  return null;
}

// Function to save the content of an HTML input to a blob
function saveHtmlInputToBlob(inputFile: HTMLInputElement): Blob | null {
  let file = inputFile.files?.[0];
  if (file) {
    let reader = new FileReader();
    reader.onload = (event: ProgressEvent<FileReader>) => {
      // Do something with the loaded content, for example save it or use it
      // further
      let content = event.target?.result as string;
      let blob = new Blob([content], { type: "text/html" });

      // console.debug("Saved Blob");

      return blob;
    };
    reader.readAsText(file);
  }

  return null;
}
