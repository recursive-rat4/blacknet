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
    const menu = $('.main-menu'), panel = $('.rightpanel'), apiVersion = "/api/v1", body = $("body");;
    const hash = localStorage.hashIndex || 'overview';
    const dialogPassword = $('.dialog.password'), mask = $('.mask');
    const account = localStorage.account;
    const dialogAccount = $('.dialog.account');


    Blacknet.init = async function () {

        await Blacknet.wait(1000);

        if (account) {

            mask.removeClass('init').hide();
            dialogAccount.hide();

            $('.overview').find('.overview_account').text(account);

            if (localStorage.isStaking) {
                $('.isStaking').text(localStorage.isStaking);
            }

            mask.on('click', function () {
                mask.hide();
                dialogPassword.hide();
            });
        } else {
            dialogAccount.find('.spinner').hide();
            dialogAccount.find('.account-input').show();
            dialogAccount.find('.enter').unbind().on('click', function () {

                let account = dialogAccount.find('.account_text').val();

                if (/^blacknet[a-z0-9]{59}$/.test(account)) {
                    localStorage.account = account;
                    location.reload();
                } else {
                    alert('Invalid Account');
                }
            });
        }


    };

    Blacknet.balance = async function () {

        let balance = $('.overview_balance');

        $.getJSON(apiVersion + '/ledger/get/' + account + '/', function (data) {
            balance.html(new BigNumber(data.balance).dividedBy(1e8) + ' BLN');
        }).fail(function () {
            balance.html('0.00000000 BLN');
        });
    };

    Blacknet.network = async function () {
        let network = $('.network');
        let data = await Blacknet.getPromise('/ledger', 'json');

        Blacknet.height = data.height;
        network.find('.height').html(data.height);
        network.find('.supply').html(new BigNumber(data.supply).dividedBy(1e8).toFixed(0));
        network.find('.accounts').html(data.accounts);
        Blacknet.renderOverview(data);

        let nodeinfo = await Blacknet.getPromise('/nodeinfo', 'json');
        network.find('.connections').text(nodeinfo.outgoing);
        $('.overview_agent').text(nodeinfo.agent);
        getPeerInfo();
    };

    Blacknet.renderOverview = function (ledger) {

        for (let key in ledger) {

            let value = ledger[key];
            if(key == 'blockTime'){
                value = Blacknet.unix_to_local_time(value);
            }else if(key == 'supply'){
                value = new BigNumber(value).dividedBy(1e8) + ' BLN';
            }
            $('.overview_' + key).text(value);
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

    Blacknet.postPromise = function (url) {
        return $.post(apiVersion + url, {});
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

        let tmpl = `<tr><td class="narrow height">${height}</td>
                    <td class="size narrow">${block.size}</td>
                    <td class="time narrow">${Blacknet.unix_to_local_time(block.time)}</td>
                    <td class="txns narrow">${block.transactions.length}</td>
                    <td class="generator">${block.generator}</td></tr>`;

        $(tmpl).prependTo("#block-list");

        let rowsCount = $('#block-list').find('tr').length;

        if (rowsCount > 35) {
            $('#block-list tr:last-child').remove();
        }
    }

    Blacknet.initRecentBlocks = async function () {

        let i = 35;
        let height = Blacknet.height;

        while (i-- > 0) {
            await Blacknet.addBlockWithHeight(height - i);
        }
    }

    Blacknet.addBlockWithHeight = async function (height) {
        let hash = await Blacknet.getPromise('/blockdb/getblockhash/' + height);
        await Blacknet.addBlock(hash, height);
    };


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
        peers.map(renderPeer);
    }

    function renderPeer(peer, index) {

        let tmpl = `<tr>
                        <td>${index + 1}</td>
                        <td class="right">${peer.remoteAddress}</td>
                        <td>${peer.agent}</td>
                        <td class="right">${peer.ping}ms</td>
                        <td class="narrow">${peer.timeOffset}</td>
                        <td class="narrow">${peer.totalBytesRead}</td>
                        <td class="narrow">${peer.totalBytesWritten}</td>
                    </tr>`;

        $(tmpl).appendTo("#peer-list");
    }

    Blacknet.ready = async function (callback) {

        Blacknet.init();
        await Blacknet.network();
        await Blacknet.initRecentBlocks();
        await Blacknet.balance();
        Blacknet.startHeight = Blacknet.height + 1;
        callback();
    };





    window.Blacknet = Blacknet;
}();