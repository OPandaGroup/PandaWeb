if (!("WebSocket" in window)){
    alert("您的浏览器不支持websocket!我们无法和服务器建立连接!");
}
var ws = new WebSocket("ws://154.64.230.230:3012");
var data = "";var connect = false;var json ;var cmd = "";
ws.onopen = function(){
    console.log("websocket连接成功!");
    ws.send("news");
    cmd = "news";
    connect = true;
};
ws.onmessage = function(e){
    if (e.data == "END"){
        console.log("服务器消息汇报结束!")
        return ;
    }
    data = e.data;
    console.log(data);
    if(cmd == "news" ){
        json = JSON.parse(data);
        console.debug(json);
    }
};
ws.onclose = function(){
    console.log("websocket连接关闭!");
};

const back = document.querySelector('#back');
console.log(back);
back.addEventListener(
    "scroll", function(){
        console.log("scroll");
        const n = document.documentElement.scrollTop;
        back.style.opacity = n >= 100 ? 1 : 0;
    }
)

back.addEventListener(
    "click", function(){
        window.scrollTo(0,0);
        console.log("click")
        document.documentElement.scrollTop = 0
    }
)

function login(){
    window.location.href="./html/login.html";
}

function register(){
    window.location.href="./html/register.html";
}
