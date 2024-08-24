function browserInfo(){
    var userAgent = navigator.userAgent;
    var isOpera = userAgent.indexOf("Opera") > -1;
    if (isOpera) {      //Opera
        return "Opera"
    }
    if (userAgent.indexOf("Firefox") > -1) {     //Firefox
        return "FF";
    }
    if (userAgent.indexOf("Chrome") > -1){   //Chrome
        return "Chrome";
    }
    if (userAgent.indexOf("Safari") > -1) {     //Safari
        return "Safari";
    }
    if (userAgent.indexOf("compatible") > -1 && userAgent.indexOf("MSIE") > -1 && !isOpera) {
        return "IE";
    }        //IE
}

if(browserInfo() == "Safari"){
    alert("我们建议您使用Chrome浏览器访问本网站,因为本网站中所有的Css都是基于Firefox编写的,在Safari中可能会出现一些问题,给您带来不便,我们深感抱歉")
}