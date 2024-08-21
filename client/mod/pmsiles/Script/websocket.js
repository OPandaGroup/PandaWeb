class WSClient{
    constructor(url, onopen, onmessage, onclose){
        this.url = url;
        this.contest = Array(String) ;
        let ws = new WebSocket(url);
        ws.onopen = onopen;
        this.onmessage_usr = onmessage;
        ws.onmessage = this.onmessage;
        ws.onclose = onclose;
    }

    onmessage(msg){
        this.onmessage_usr(msg);
        this.contest.push(msg);
    }

    send(msg){
        this.ws.send(msg);
    }
}