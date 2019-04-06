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
        $.getJSON(apiVersion + '/ledger', function (data) {

            network.find('.height').html(data.height);
            network.find('.supply').html(new BigNumber(data.supply).dividedBy(1e8).toFixed(0));
            network.find('.accounts').html(data.accounts);
        });

        $.getJSON(apiVersion + '/nodeinfo', function (data) {

            network.find('.connections').html(data.incoming);
        });
    };
    Blacknet.get = function (url, callback) {

        return $.get(apiVersion + url, callback);
    };
    Blacknet.getPromise = function (url) {

        return $.get(apiVersion + url);
    };
    Blacknet.post = function (url, callback, type) {
        return $.post(apiVersion + url, {}, callback, type);
    };

    Blacknet.sendMoney = function (mnemonic, amount, to, message, encrypted, callback) {

        let fee = 100000, amountText, url;

        amountText = new BigNumber(amount).toFixed(8);
        amount = new BigNumber(amount).times(1e8);

        url = "/transfer/" + mnemonic + "/" + fee + "/" + amount + "/" + to + "/" + message + "/" + encrypted + "/";

        if (confirm('Are you sure you want to send?\n\n' + amountText + ' BLN to \n' + to + '\n\n0.001 BLN added as transaction fee?')) {

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


        if (confirm('Are you sure you want to ' + type_text + '?\n\n' + amountText + ' BLN to \n' + to + '\n\n0.001 BLN added as transaction fee?')) {

            Blacknet.post(url, callback);
        }
    };
    Blacknet.wait = function(timeout){
        return new Promise(function(resolve, reject){
            setTimeout(function(){
                resolve();
            }, timeout);
        });
    };

    Blacknet.init();
    Blacknet.network();
    Blacknet.balance();
    window.Blacknet = Blacknet;
}();