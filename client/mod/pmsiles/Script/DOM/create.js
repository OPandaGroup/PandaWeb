function createDOM(...args) {
    args.forEach(element => {
        if (element.type == "div") {
            let div = document.createElement("div");
            div.className = element.className;
            div.id = element.id;
            div.innerHTML = element.innerHTML;
            document.body.appendChild(div);
        }
        if (element.type == "button") {
            let button = document.createElement("button");
            button.className = element.className;
            button.id = element.id;
            button.innerHTML = element.innerHTML;
            document.body.appendChild(button);
        }
    })
    
}