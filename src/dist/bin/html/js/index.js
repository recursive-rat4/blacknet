void function(){

    const apiVersion = "/api/v1";

    function request(method, url, callback) {
        
        let xhr = new XMLHttpRequest();
        
        url = apiVersion + url;

        xhr.onreadystatechange = callback;
        xhr.open(method, url, true);
        xhr.send();
    }

    function getJSON(url, callback){

        $.getJSON(apiVersion + url, callback);
    }


    function start_staking() {
        let mnemonic = $('#start_staking_mnemonic').val();
        let url = "/staker/start/" + mnemonic + "/";

        request("POST", url, function () {
            $('#start_staking_result').val(this.responseText);
        });
    }

    function balance() {
        let account = $('#balance_account').val();
        let url = "/ledger/get/" + account + "/";

        getJSON( url, function (data) {
            console.log(this.status)
            if (this.status == 404) {
                $('#balance_result').val(0)
                return;
            }
            let data = JSON.parse(this.responseText);
            $('#balance_result').val(data.balance);
        });
    }
    function transfer() {
        let mnemonic = $('#transfer_mnemonic').val();
        let fee = $('#transfer_fee').val();
        let amount = $('#transfer_amount').val();
        let to = $('#transfer_to').val();
        let message = $('#transfer_message').val();
        let encrypted = message && $('#transfer_encrypted').prop('checked') ? "1" : "";
        let url = "/transfer/" + mnemonic + "/" + fee + "/" + amount + "/" + to + "/" + message + "/" + encrypted + "/";

        request("POST", url, function () {
            $('#transfer_result').val(this.responseText);
        });
    }
    function sign() {
        let mnemonic = $('#sign_mnemonic').val();
        let message = $('#sign_message').val();
        let url = "/signmessage/" + mnemonic + "/" + message + "/";

        request("POST", url, function () {
            $('#sign_result').val(this.responseText);
        });
    }
    function verify() {
        let account = $('#verify_account').val();
        let signature = $('#verify_signature').val();
        let message = $('#verify_message').val();
        let url = "/verifymessage/" + account + "/" + signature + "/" + message + "/";

        request("GET", url, function () {
            $('#verify_result').val(this.responseText);
        });
    }
    function mnemonic_info() {
        let mnemonic = $('#mnemonic_info').val();
        let url = "/mnemonic/info/" + mnemonic + "/";

        request("POST", url, function () {
            $('#mnemonic_info_result').val(this.responseText);
        });
    }
    function request_info() {
        getJSON("/ledger", function (data) {
            $('#info_height').html("Blockchain height: " + data.height);
        });
        getJSON("/nodeinfo", function (data) {
            $('#info_connections').html("Connections: " + data.outgoing + " outgoing, " + data.incoming + " incoming");
        });
    }
    request_info();

    
    $('#start_staking').on('click', start_staking);
    $('#balance').on('click', balance);
    $('#transfer').on('click', transfer);
    $('#sign').on('click', sign);
    $('#verify').on('click', verify);
    $('#mnemonic_info').on('click', mnemonic_info);

    
    let ws = new WebSocket("ws://"+location.host+"/api/v1/notify/block");
    ws.onmessage = request_info;
}();