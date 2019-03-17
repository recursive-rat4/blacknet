$(document).ready(function(){

    const apiVersion = "/api/v1", body = $("body");
    function request_promise(method, url) {
        return new Promise(function (resolve, reject) {
            url = apiVersion + url;
            const xhr = new XMLHttpRequest();
            xhr.open(method, url, true);
            xhr.onload = function () {
                if (this.status >= 200 && this.status < 300) {
                    resolve(xhr.responseText);
                } else {
                    reject({
                        status: this.status,
                        statusText: xhr.statusText
                    });
                }
            };
            xhr.onerror = function () {
                reject({
                    status: this.status,
                    statusText: xhr.statusText
                });
            };
            xhr.send();
        });
    }
    function request(method, url, callback) {
        url = apiVersion + url
        let xhr = new XMLHttpRequest();
        xhr.addEventListener('load', callback);
        xhr.open(method, url, true);
        xhr.send();
    }
    function start_staking() {
        let mnemonic = document.getElementById('start_staking_mnemonic').value;
        let url = "/staker/start/" + mnemonic + "/";

        request("POST", url, function () {
            document.getElementById('start_staking_result').value = this.responseText;
        });

        url = "/mnemonic/info/" + mnemonic + "/";
        request("POST", url, function () {
            const data = JSON.parse(this.responseText);
            data.mnemonic = '[hidden]';
            document.getElementById('mnemonic_info_result').value = JSON.stringify(data);
            document.getElementById('balance_account').value = data.address;
            balance();
        });
    }
    function stop_staking() {
        let mnemonic = document.getElementById('start_staking_mnemonic').value;
        let url = "/staker/stop/" + mnemonic + "/";

        request("POST", url, function () {
            document.getElementById('start_staking_result').value = this.responseText;
        });

        url = "/mnemonic/info/" + mnemonic + "/";
        request("POST", url, function () {
            const data = JSON.parse(this.responseText);
            data.mnemonic = '[hidden]';
            document.getElementById('mnemonic_info_result').value = JSON.stringify(data);
            document.getElementById('balance_account').value = data.address;
            balance();
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
        let mnemonic = document.getElementById('mnemonic_info_mnemonic').value;
        let url = "/mnemonic/info/" + mnemonic + "/";

        request("POST", url, function () {
            document.getElementById('mnemonic_info_result').value = this.responseText;
        });
    }
    async function display_api_json_result(apiCall) {
        if (apiCall === '') return;
        const url = `/${apiCall}/`;
        const textData = await request_promise("GET", url);
        const jsonData = JSON.parse(textData);
        const pre = document.getElementById("api-json-result");
        const h2 = document.createElement('h2');
        const p = document.createElement('p');
        pre.innerHTML = '';
        h2.innerHTML = apiCall;
        p.innerHTML = '[Close]';
        p.style.cursor = 'pointer';
        p.onclick = function () { pre.innerHTML = '' };
        pre.append(h2);
        pre.append(p);
        pre.append(textData);
        switch (apiCall) {
            case "nodeinfo":
                document.getElementById('info_connections').innerHTML = "Connections: " + jsonData.outgoing + " outgoing, " + jsonData.incoming + " incoming";
                break;
            case "ledger":
                document.getElementById('info_height').innerHTML = "Blockchain height: " + jsonData.height;
                break;
            case "peerinfo":
                display_node_list(jsonData);
                break;
        }
    }
    function display_node_list(data) {
        const tbody = document.querySelector('#node-list');
        const template = document.querySelector('#node-row');
        tbody.innerHTML = '';
        data.forEach((node) => {
            const clone = document.importNode(template.content, true);
            const td = clone.querySelectorAll('td');
            td[0].textContent = node.remoteAddress;
            td[1].textContent = node.ping;
            td[2].textContent = node.totalBytesRead;
            td[3].textContent = node.totalBytesWritten;
            td[4].textContent = node.state === 'INCOMING_CONNECTED' ? 'In' : 'Out';
            tbody.appendChild(clone);
        });
    }
    async function add_block(hash, height) {
        const url = `/blockdb/get/${hash}`;
        let blockData = await request_promise('GET', url);
        blockData = JSON.parse(blockData);
        const table = document.getElementById("block-table");
        const rowCount = table.rows.length;
        rowCount > 10 ? table.deleteRow(rowCount - 1) : false;
        const tbody = document.querySelector('#block-list');
        const template = document.querySelector('#block-row');
        const clone = document.importNode(template.content, true);
        const td = clone.querySelectorAll('td');
        td[0].textContent = height;
        td[1].textContent = hash;
        td[2].textContent = blockData.size;
        td[3].textContent = unix_to_local_time(blockData.time);
        td[4].textContent = blockData.transactions.length;
        td[5].textContent = blockData.generator;
        tbody.prepend(clone);
    }
    function unix_to_local_time(unix_timestamp) {
        const date = new Date(unix_timestamp * 1000);
        const hours = date.getHours();
        const minutes = "0" + date.getMinutes();
        const seconds = "0" + date.getSeconds();
        return hours + ':' + minutes.substr(-2) + ':' + seconds.substr(-2);
    }
    async function request_info(message = {}) {
        let ledgerData = await request_promise("GET", "/ledger");
        ledgerData = JSON.parse(ledgerData);
        document.getElementById('info_height').innerHTML = "Blockchain height: " + ledgerData.height;

        let nodeInfoData = await request_promise("GET", "/nodeinfo");
        nodeInfoData = JSON.parse(nodeInfoData);
        document.getElementById('info_connections').innerHTML = "Connections: " + nodeInfoData.outgoing + " outgoing, " + nodeInfoData.incoming + " incoming";

        let peerInfoData = await request_promise("GET", "/peerinfo");
        peerInfoData = JSON.parse(peerInfoData);
        display_node_list(peerInfoData);

        if (message.data) { add_block(message.data, ledgerData.height); }
    }

    async function generate_new_account() {
        let url = '/account/generate';
        let blockData = await request_promise('GET', url);
        blockData = JSON.parse(blockData);
        document.getElementById('new_account').value = blockData.address;
        document.getElementById('new_mnemonic').value = blockData.mnemonic;
    }

    body.on("click", "#start_staking", start_staking)
        .on("click", "#stop_staking", stop_staking)
        .on("click", "#balance", balance)
        .on("click", "#transfer", transfer)
        .on("click", "#sign", sign)
        .on("click", "#verify", verify)
        .on("click", "#mnemonic_info", mnemonic_info)
        .on("click", "#generate_new_account", generate_new_account)
        .on("click", "#display_api_json_result",function(event){
            let el = event.target;
            display_api_json_result(el.dataset.type);
        });


    request_info();
    let ws = new WebSocket("ws://" + location.host + "/api/v1/notify/block");
    ws.onmessage = request_info;

})
