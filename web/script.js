function changeColor() {
  console.log("Cliecked\n")
  let input = document.createElement('input');
  input.type = 'file';
  input.onchange = _ => {

  console.log("selected\n")
    // you can use this method to get file and perform respective operations
            let files =   Array.from(input.files);
            console.log(files);
        };
  input.click();

  let asd = document.createElement("p");
  asd.innerText = "Some paragraph xd";
  document.body.appendChild(asd);
}
