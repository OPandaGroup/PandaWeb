function createDOM(element, parent, style) {
    const DomElement = document.createElement(element);
    DomElement.style = style;
    if(parent == undefined){
        document.body.appendChild(DomElement);
    }else{
        parent.appendChild(DomElement);
    }
    return DomElement;
}

function appendDOM(parent, child){
    parent.appendChild(child);
}

function hiddenDOM(element){
    element.style.display = "none";
}

function showDOM(element){
    element.style.display = "block";
}

function isShowDOM(element){
    return (element.style.display == "block");
}

function removeDOM(element){
    element.remove();
}

function setStyle(element, style){
    element.style = style;
}

function setAttribute(element, attribute, value){
    element.setAttribute(attribute, value);
}

function findIdDOM(id){
    return document.getElementById(id);
}

function findClassDOM(className){
    return document.getElementsByClassName(className);
}

function isVoid(element){
    return (element == undefined);
}