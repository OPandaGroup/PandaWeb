function setStyle(id, key, value){
    const style = document.getElementById(id);
    style.setAttribute(key, value)
}

function setStyleConst(id, key, value){
    id.setAttribute(key, value);
}

function readDict(dict, id){
    for (const [key, value] of Object.entries(dict)) {
        setStyleConst(id, key, value);
    }
}