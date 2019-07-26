/*
 * Copyright (c) 2018-2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

void function () {

    const Blacknet = {};
    const DEFAULT_CONFIRMATIONS = 10;
    const blockListEl = $('#block-list'), apiVersion = "/api/v1", body = $("body");;
    const progressStats = $('.progress-stats, .progress-stats-text');
    const dialogPassword = $('.dialog.password'), mask = $('.mask');
    const account = localStorage.account;
    const dialogAccount = $('.dialog.account');
    const notificationNode = $('.notification.tx').first();
    const txList = $('#tx-list');

    /**
     * init 
     * @method init
     * @for Blacknet
     * @param {null} 
     * @return {null}
     */
    Blacknet.init = async function () {

        await Blacknet.wait(1000);
        
        if (account) {

            mask.removeClass('init').hide();
            dialogAccount.hide();
            Blacknet.showProgress();

            $('.overview').find('.overview_account').text(account);

            if (localStorage.isStaking) {
                $('.is_staking').text(localStorage.isStaking);
            }

            mask.on('click', function () {
                mask.hide();
                dialogPassword.hide();
            });
        } else {
            dialogAccount.find('.spinner').hide();
            dialogAccount.find('.account-input').show();
            dialogAccount.find('.enter').unbind().on('click', async function () {

                let account = dialogAccount.find('.account_text').val();

                if (account.length < 22) {
                    return;
                }
                if (/^blacknet[a-z0-9]{59}$/.test(account)) {
                    localStorage.account = account;
                } else {
                    account = await Blacknet.mnemonicToAddress(account);
                    localStorage.account = account;
                }
                location.reload();
            });
        }
    };

    Blacknet.mnemonicToAddress = async function (mnemonic) {
        let url = "/mnemonic/info/" + mnemonic + "/";
        let mnemonicInfo = await Blacknet.postPromise(url, true);
        mnemonicInfo = JSON.parse(mnemonicInfo);
        return mnemonicInfo.address;
    };

    /**
     * account balance
     * @method balance
     * @for Blacknet
     * @param {null} 
     * @return {null}
     */
    Blacknet.balance = async function () {

        let balance = $('.overview_balance'),
            confirmedBalance = $('.overview_confirmed_balance'),
            stakingBalance = $('.overview_staking_balance');;

        $.getJSON(apiVersion + '/ledger/get/' + account + '/', function (data) {
            balance.html(Blacknet.toBLNString(data.balance));
            confirmedBalance.html(Blacknet.toBLNString(data.confirmedBalance));
            stakingBalance.html(Blacknet.toBLNString(data.stakingBalance));

        }).fail(function () {
            balance.html('0.00000000 BLN');
        });
    };

    Blacknet.toBLNString = function (number) {
        return new BigNumber(number).dividedBy(1e8).toFixed(8) + ' BLN';
    };



    Blacknet.renderStatus = function () {

        let network = $('.network');
        let ledger = Blacknet.ledger, nodeinfo = Blacknet.nodeinfo;

        network.find('.height').html(ledger.height);
        network.find('.supply').html(new BigNumber(ledger.supply).dividedBy(1e8).toFixed(0));
        network.find('.accounts').html(ledger.accounts);
        network.find('.connections').text(nodeinfo.outgoing + nodeinfo.incoming);
        $('.overview_version').text(nodeinfo.version);
    };

    Blacknet.renderOverview = function () {

        let ledger = Blacknet.ledger;

        for (let key in ledger) {

            let value = ledger[key];
            if (key == 'blockTime') {
                value = Blacknet.unix_to_local_time(value);
                Blacknet.renderProgressBar(ledger[key]);
            } else if (key == 'supply') {
                value = new BigNumber(value).dividedBy(1e8) + ' BLN';
            }
            $('.overview_' + key).text(value);
        }
    };

    Blacknet.renderProgressBar = async function (timestamp) {

        let secs = Date.now() / 1000 - timestamp, totalSecs, pecent, timeBehindText = "", now = Date.now();
        let HOUR_IN_SECONDS = 60 * 60;
        let DAY_IN_SECONDS = 24 * 60 * 60;
        let WEEK_IN_SECONDS = 7 * 24 * 60 * 60;
        let YEAR_IN_SECONDS = 31556952; // Average length of year in Gregorian calendar

        if (secs < 5 * 60) {
            timeBehindText = undefined;
        } else if (secs < 2 * DAY_IN_SECONDS) {
            timeBehindText = (secs / HOUR_IN_SECONDS).toFixed(2) + " hour(s)";
        } else if (secs < 2 * WEEK_IN_SECONDS) {
            timeBehindText = (secs / DAY_IN_SECONDS).toFixed(2) + " day(s)";
        } else if (secs < YEAR_IN_SECONDS) {
            timeBehindText = (secs / WEEK_IN_SECONDS).toFixed(2) + " week(s)";
        } else {
            let years = secs / YEAR_IN_SECONDS;
            let remainder = secs % YEAR_IN_SECONDS;
            timeBehindText = years.toFixed(2) + " year(s) and " + remainder.toFixed(2) + "week(s)";
        }

        if (!Blacknet.startTime) {

            const GENESIS_TIME = 1545555600;
            Blacknet.startTime = GENESIS_TIME;
        }
        Blacknet.timeBehindText = timeBehindText;

        if (timeBehindText == undefined) {
            progressStats.hide();
            return;
        }

        totalSecs = Date.now() / 1000 - Blacknet.startTime;
        pecent = (secs * 100) / totalSecs;

        pecent = 100 - pecent;

        $('.progress-bar').css('width', `${pecent}%`);
        $('.progress-stats-text').text(timeBehindText + " behind");
    }

    Blacknet.showProgress = function () {

        if (Blacknet.timeBehindText != undefined) {
            progressStats.show();
        }
    };

    Blacknet.get = function (url, callback) {

        return $.get(apiVersion + url, callback);
    };

    Blacknet.getPromise = function (url, type) {

        return type == 'json' ? $.getJSON(apiVersion + url) : $.get(apiVersion + url);
    };
    Blacknet.post = function (url, callback, type) {
        return $.post(apiVersion + url, {}, callback, type);
    };

    Blacknet.postPromise = function (url, isNeedAlert) {
        return $.post(apiVersion + url, {}).fail(function (res) {
            if (isNeedAlert && res.responseText) alert(res.responseText);
        });
    };

    Blacknet.sendMoney = function (mnemonic, amount, to, message, encrypted, callback) {

        let fee = 100000, amountText, url;

        amountText = new BigNumber(amount).toFixed(8);
        amount = new BigNumber(amount).times(1e8);

        url = "/transfer/" + mnemonic + "/" + fee + "/" + amount + "/" + to + "/" + message + "/" + encrypted + "/";

        if (confirm('Are you sure you want to send?\n\n' + amountText + ' BLN to \n' +
            to + '\n\n0.001 BLN added as transaction fee?')) {

            Blacknet.post(url, callback);
        }
    };

    Blacknet.lease = function (mnemonic, type, amount, to, height, callback) {

        let fee = 100000, amountText, url, type_text = type == 'lease' ? 'lease' : 'cancel lease';

        amountText = new BigNumber(amount).toFixed(8);
        amount = new BigNumber(amount).times(1e8);

        if (type == 'lease') {
            url = "/" + type + "/" + mnemonic + "/" + fee + "/" + amount + "/" + to + "/";
        } else {
            url = "/cancellease/" + mnemonic + "/" + fee + "/" + amount + "/" + to + "/" + height + "/";
        }

        if (confirm('Are you sure you want to ' + type_text + '?\n\n' + amountText +
            ' BLN to \n' + to + '\n\n0.001 BLN added as transaction fee?')) {

            Blacknet.post(url, callback);
        }
    };

    Blacknet.wait = function (timeout) {
        return new Promise(function (resolve, reject) {
            setTimeout(function () {
                resolve();
            }, timeout);
        });
    };

    Blacknet.unix_to_local_time = function (unix_timestamp) {

        let date = new Date(unix_timestamp * 1000);
        let hours = "0" + date.getHours();
        let minutes = "0" + date.getMinutes();
        let seconds = "0" + date.getSeconds();
        let day = date.getDate();
        let year = date.getFullYear();
        let month = date.getMonth() + 1;

        return year + "-" + ('0' + month).substr(-2) + "-" +
            ('0' + day).substr(-2) + " " + hours.substr(-2) + ':' + minutes.substr(-2) + ':' + seconds.substr(-2);
    }

    Blacknet.addBlock = async function (hash, height) {

        let url = `/blockdb/get/${hash}`;
        let block = await Blacknet.getPromise(url, 'json');

        block.txns = block.transactions.length;
        Blacknet.template.block(blockListEl, block, height, false);

        return block.previous
    }

    Blacknet.initRecentBlocks = async function () {

        let i = 0;
        let hash = Blacknet.ledger.blockHash;
        let height = Blacknet.ledger.height;

        if (height < 36) return;

        while (i++ < 35) {
            hash = await Blacknet.addBlock(hash, height);
            height--;
        }
    }

    Blacknet.serializeTx = function (transactions) {

        let tx, txs = [];

        while (transactions.length) {

            let tmp = transactions.shift();

            if (typeof tmp == 'string') {

                tx = { hash: tmp };
            } else {
                tx.height = tmp.height;
                tx.time = tmp.time;
                txs.push(tx);
            }
        }

        txs.sort(function (x, y) {
            return y.height - x.height;
        });
        return txs;
    }

    Blacknet.initRecentTransactions = async function () {

        let data = await Blacknet.getPromise('/walletdb/getwallet/' + account, 'json');
        let transactions = data.transactions;
        let array = [];
        
        array = Blacknet.serializeTx(transactions);
        Blacknet.txdb = {};
        Blacknet.txIndex = array;
        Blacknet.renderLeaseOption(array);

        await Blacknet.renderTxs(array);
        Blacknet.renderTxOption();
    };

    Blacknet.renderTxOption = function(){

        // let txTypeNode = $('#tx-type'), list = Blacknet.txIndex, txCount = {};

        // txTypeNode.find('option[value="rat4"]').text('All ('+list.length + ')');

        // for(let tx of Blacknet.txIndex){

        //     if(txCount[tx.type])
        // }


    };

    Blacknet.renderTxs = async function(txArray){

        let defaultTxAmount = 100, txProgress = $('.tx-progress'),
            showMore = $('.tx-foot .show_more_txs'),
            noTxYet = $('.tx-foot .no_tx_yet');

        txArray.length == 0 ? noTxYet.show() : noTxYet.hide();
        txProgress.hide();
        txList.html('');

        Blacknet.currentTxIndex = txArray;

        for(let tx of txArray){

            if(defaultTxAmount-- < 1){
                break;
            }
            await Blacknet.processTransaction(tx);
        }

        if ( defaultTxAmount < 0) {
            showMore.show();
        }

    };

    Blacknet.processTransaction = async function (data) {

        let tx = Blacknet.txdb[data.height + data.time];

        if (tx) {
            await Blacknet.renderTransaction(tx);
            return;
        }

        tx = await Blacknet.getPromise('/walletdb/gettransaction/' + data.hash + '/false', 'json');
        tx.height = data.height;
        tx.time = data.time;
        Blacknet.txdb[data.height + data.time] = tx;
        await Blacknet.renderTransaction(tx);
    };

    Blacknet.showMoreTxs = async function () {

        let transactions = Blacknet.currentTxIndex.slice(100);
        let node = $('.tx-item:last-child');

        time = +node[0].dataset.time || 0;

        for(let tx of transactions){
            await Blacknet.processTransaction(tx);
        }
    };

    Blacknet.renderLeaseOption = async function (txns) {


        let outLeases = await Blacknet.getPromise('/walletdb/getoutleases/' + account, 'json');
        let accounts = [], aobj = {}, hobj = {}, height = [];

        if (outLeases.length == 0) return;

        outLeases.map(function (tx) {
            aobj[tx.publicKey] = '';
            hobj[tx.height] = '';
        });

        accounts = Object.keys(aobj);
        height = Object.keys(hobj);

        accounts.forEach(function (account) {

            $('#cancel_lease_to').append($("<option></option>").attr("value", account).text(account));
        });

        height.forEach(function (account) {

            $('#cancel_lease_height').append($("<option></option>").attr("value", account).text(account));
        });

        $('.cancel_lease_tab').show();
    };

    Blacknet.renderTransaction = async function (tx, prepend) {

        let node = txList.find('.txhash' + tx.height + tx.time);
        if (typeof tx.height == 'undefined') {
            tx.height = 0;
        }
        // if tx already render, update status
        if (node.html()) {
            await Blacknet.renderTxStatus(0, node[0]);
            return;
        }

        node = await Blacknet.template.transaction(tx, account);
        prepend ? node.prependTo(txList) : node.appendTo(txList);
    };

    Blacknet.getStatusText = async function (height, hash) {

        let confirmations = Blacknet.ledger.height - height + 1, statusText = 'Confirmed';
        if (height == 0) {

            confirmations = await Blacknet.getPromise('/walletdb/getconfirmations/' + hash);
            statusText = `${confirmations} Confirmations`;

        } else if (confirmations < DEFAULT_CONFIRMATIONS) {

            statusText = `${confirmations} Confirmations`;
        }
        return statusText;
    };

    Blacknet.getTxType = function (tx) {

        let txType = ["Transfer", "Burn", "Lease", "CancelLease", "Bundle", "CreateHTLC",
            "UnlockHTLC", "RefundHTLC", "SpendHTLC", "CreateMultisig", "SpendMultisig"];

        let type = txType[tx.type];

        if (tx.type == 254) {
            type = 'Generated';
        }

        if (tx.type == 0) {
            if (tx.from == account) {
                type = "Sent to";
            } else {
                type = "Received from";
            }
        }
        return type;
    };

    Blacknet.getFormatBalance = function (balance) {

        return new BigNumber(balance).dividedBy(1e8).toFixed(8) + ' BLN';
    };

    Blacknet.newTransactionNotify = function (tx) {

        let notification = notificationNode.clone();
        let time = Blacknet.unix_to_local_time(tx.time);
        let type = Blacknet.getTxType(tx), amount = Blacknet.getFormatBalance(tx.data.amount);;

        if (type == 'Generated') {
            amount = Blacknet.getFormatBalance(tx.fee);
        }

        notification.find('.time').text(time);
        notification.find('.type').text(type);
        notification.find('.amount').text(amount);

        notification.appendTo('body').show();

        notification.delay(2000).animate({ top: "-100px", opacity: 0 }, 1000, function () {
            notification.remove();
        });
    };
    Blacknet.renderTxStatus = async function (index, el) {

        let statusText, node = $(el).find('.status');

        if (node.text() == 'Confirmed') return;

        statusText = await Blacknet.getStatusText(el.dataset.height, el.dataset.hash);
        node.text(statusText);
    };



    Blacknet.refreshTxConfirmations = function () {

        $.each($('#tx-list tr'), Blacknet.renderTxStatus);
    };

    Blacknet.renderBlock = async function (block, height, prepend = true) {

        Blacknet.template.block(blockListEl, block, height, prepend);
    }

    Blacknet.throttle = function (fn, threshhold = 250) {

        let last, timer;

        return function () {

            let context = this;
            let args = arguments;
            let now = +new Date();

            if (last && now < last + threshhold) {
                clearTimeout(timer);

                timer = setTimeout(function () {
                    last = now;
                    fn.apply(context, args);
                }, threshhold);

            } else {
                last = now;
                fn.apply(context, args);
            }
        }
    }

    async function getPeerInfo() {

        let peers = await Blacknet.getPromise('/peerinfo', 'json');
        $('#peer-list').html('');
        peers.map(Blacknet.template.peer);
    }


    Blacknet.ready = async function (callback) {

        let lang = navigator.language || navigator.userLanguage;
        if (lang.indexOf('zh') !== -1) {
            i18n({ locale: 'zh' });
        } else if (lang.indexOf('ja') !== -1) {
            i18n({ locale: 'ja' });
        } else if (lang.indexOf('sk') !== -1) {
            i18n({ locale: 'sk' });
        }

        Blacknet.init();
        await Blacknet.balance();
        await Blacknet.network();
        await Blacknet.initRecentBlocks();

        if (account) {
            await Blacknet.initRecentTransactions();
        }

        callback();
    };

    Blacknet.refreshBalance = async function () {

        await Blacknet.balance();
        // if (account) {
        //     await Blacknet.initRecentTransactions();
        // }
    };

    const timePeerInfo = Blacknet.throttle(getPeerInfo, 1000);
    Blacknet.network = async function () {

        Blacknet.ledger = await Blacknet.getPromise('/ledger', 'json');
        Blacknet.nodeinfo = await Blacknet.getPromise('/nodeinfo', 'json');

        Blacknet.renderStatus();
        Blacknet.renderOverview();

        timePeerInfo();
    };

    /**
     * verify mnemonic
     * @method verifyMnemonic
     * @for Blacknet
     * @param {string} mnemonic
     * @return {boolean} true/fasle
     */
    Blacknet.verifyMnemonic = function(mnemonic){
        if(Object.prototype.toString.call(mnemonic) === "[object String]" && mnemonic.split(" ").length == 12){
            return true
        }
        return false
    }
    /**
     * verify account address
     * @method verifyAccount
     * @for Blacknet
     * @param {string} account
     * @return {boolean} true/fasle
     */
    Blacknet.verifyAccount = function(account){
        if(Object.prototype.toString.call(account) === "[object String]" && account.length > 21 && /^blacknet[a-z0-9]{59}$/.test(account)){
            return true
        }
        return false
    }
    /**
     * verify amount
     * @method verifyAmount
     * @for Blacknet
     * @param {string} amount
     * @return {boolean} true/fasle
     */
    Blacknet.verifyAmount = function(amount){
        if(Object.prototype.toString.call(amount) === "[object Number]" && amount > 0){
            return true
        }
        return false
    }
    /**
     * verify message
     * @method verifyMessage
     * @for Blacknet
     * @param {string} message
     * @return {boolean} true/fasle
     */
    Blacknet.verifyMessage = function(message){
        if(Object.prototype.toString.call(message) === "[object String]" && message.length > 0){
            return true
        }
        return false
    }
    /**
     * verify sign
     * @method verifySign
     * @for Blacknet
     * @param {string} sign
     * @return {boolean} true/fasle
     */
    Blacknet.verifySign = function(sign){
        if(Object.prototype.toString.call(sign) === "[object String]" && sign.length === 128){
            return true
        }
        return false
    }
    /**
     * verify ip
     * @method verifyIP
     * @for Blacknet
     * @param {string} ip
     * @return {boolean} true/fasle
     */
    Blacknet.verifyIP = function(ip){
        if(Object.prototype.toString.call(ip) === "[object String]" && /^(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])$/.test(ip)){
            return true
        }
        return false
    }

    /**
     * message tips
     * @method message
     * @for Blacknet
     * @param {string} msg
     * @param {string} type
     * @return {null}
     */
    Blacknet.message = function(msg, type){
        Blacknet.template.message(msg, type)
    }

    window.addEventListener('beforeunload', function (e) {

        if (window.isGenerated) {

            e.preventDefault();
            e.returnValue = '';
        }
    });

    window.Blacknet = Blacknet;
}();