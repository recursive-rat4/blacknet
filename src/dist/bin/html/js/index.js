void function(){

    const apiVersion = "/api/v1";

    function request(method, url, callback) {
        
        let xhr = new XMLHttpRequest();
        
        url = apiVersion + url;

        xhr.onreadystatechange = callback;
        xhr.open(method, url, true);
        xhr.send();
    }
    function start_staking() {
        let mnemonic = document.getElementById('start_staking_mnemonic').value;
        let url = "/staker/start/" + mnemonic + "/";

        request("POST", url, function () {
            document.getElementById('start_staking_result').value = this.responseText;
        });
    }
    function balance() {
        let account = document.getElementById('balance_account').value;
        let url = "/ledger/get/" + account + "/";

        request("GET", url, function () {
            if (this.status == 404) {
                document.getElementById('balance_result').value = 0;
                return;
            }
            let data = JSON.parse(this.responseText)
            document.getElementById('balance_result').value = data.balance;
        });
    }
    function transfer() {
        let mnemonic = document.getElementById('transfer_mnemonic').value;
        let fee = document.getElementById('transfer_fee').value;
        let amount = document.getElementById('transfer_amount').value;
        let to = document.getElementById('transfer_to').value;
        let message = document.getElementById('transfer_message').value;
        let encrypted = message && document.getElementById('transfer_encrypted').checked ? "1" : "";
        let url = "/transfer/" + mnemonic + "/" + fee + "/" + amount + "/" + to + "/" + message + "/" + encrypted + "/";

        request("POST", url, function () {
            document.getElementById('transfer_result').value = this.responseText;
        });
    }
    function sign() {
        let mnemonic = document.getElementById('sign_mnemonic').value;
        let message = document.getElementById('sign_message').value;
        let url = "/signmessage/" + mnemonic + "/" + message + "/";

        request("POST", url, function () {
            document.getElementById('sign_result').value = this.responseText;
        });
    }
    function verify() {
        let account = document.getElementById('verify_account').value;
        let signature = document.getElementById('verify_signature').value;
        let message = document.getElementById('verify_message').value;
        let url = "/verifymessage/" + account + "/" + signature + "/" + message + "/";

        request("GET", url, function () {
            document.getElementById('verify_result').value = this.responseText;
        });
    }
    function mnemonic_info() {
        let mnemonic = document.getElementById('mnemonic_info').value;
        let url = "/mnemonic/info/" + mnemonic + "/";

        request("POST", url, function () {
            document.getElementById('mnemonic_info_result').value = this.responseText;
        });
    }
    function request_info() {
        request("GET", "/ledger", function () {
            let data = JSON.parse(this.responseText)
            document.getElementById('info_height').innerHTML = "Blockchain height: " + data.height;
        });
        request("GET", "/nodeinfo", function () {
            let data = JSON.parse(this.responseText)
            document.getElementById('info_connections').innerHTML = "Connections: " + data.outgoing + " outgoing, " + data.incoming + " incoming";
        });
    }
    request_info();
    
    let ws = new WebSocket("ws://"+location.hostname+"/api/v1/notify/block");
    ws.onmessage = request_info;
}();