document.write("<link rel='stylesheet' href='../Style/shadow.css'>");
document.write("<script src='../Script/style.js'></script>");
//imp

class WindowModule{
    constructor(opts){
        this.x = opts.x;
        this.y = opts.y;
        this.width = opts.width;
        this.height = opts.height;
        this.title = opts.title;
        this.closeButton = opts.closeButton;
        this.content = opts.content;
        
        if(opts.parent == undefined){
            var parent = document.body;
        }else{
            var parent = opts.parent;
        }
        this.display(parent);
        this.setContentStyle(opts.contentStyle);
        this.setWindowStyle(opts.windowStyle);
        this.setTitleStyle(opts.titleStyle);
    }

    display(parent){
        const mainWindow = document.createElement("div");
        parent.appendChild(mainWindow);
        mainWindow.style.position = "absolute";
        mainWindow.style.top = this.y;
        mainWindow.style.left = this.x;
        mainWindow.style.width = this.width;
        mainWindow.style.height = this.height;
        mainWindow.style.border = "0px solid black";
        mainWindow.style.borderRadius = "10px"; 
        mainWindow.display = "block";
        mainWindow.className = "shadow_1";
        const titleBar = document.createElement("h2");
        const content = document.createElement("main");
        this.main_Window = mainWindow;
        this.title_Bar = titleBar;
        this.content_ = content;
        titleBar.innerHTML = this.title;
        mainWindow.appendChild(titleBar);
        content.innerHTML = this.content;
        mainWindow.appendChild(content);
        content.style.font = "16px YaHei";
        titleBar.style.fontWeight = "bold";   
        if(this.closeButton){
            const closeButton = document.createElement("button");
            closeButton.style.backgroundColor = "red";
            closeButton.style.position = "absolute";
            closeButton.style.top = "0";
            closeButton.style.right = "0";
            closeButton.style.width = "20px";
            closeButton.style.height = "20px";
            closeButton.style.borderRadius = "50%";
            closeButton.style.border = "none";
            closeButton.addEventListener("mouseover", () => {
                closeButton.innerHTML = "x";
            })
            closeButton.addEventListener("mouseout", () => {
                closeButton.innerHTML = "";
            })
            closeButton.addEventListener("click", () => {
                mainWindow.style.display = "none";
            })
            mainWindow.appendChild(closeButton);
        }
        console.log(mainWindow);
    }

    setContentStyle(Style){
        this.content_.style = Style;
    }

    setTitleStyle(Style){
        this.title_Bar.style = Style;
    }

    setWindowStyle(Style){
        for(let key in Style){
            this.main_Window.style[key] = Style[key];
        }
    }

    Refresh(parent){
        this.display(parent);
    }
}