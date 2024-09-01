var ws = new WebSocket("ws://127.0.0.1:3012/main");

ws.onopen = function(){
    console.log("websocket连接成功!");
    console.log("sender send data: news to server")
    ws.send("login");
};

document.getElementById("form").onsubmit = function(){
    var name = document.getElementById("name").value;
    alert("sender send data: "+name+" to server")
}