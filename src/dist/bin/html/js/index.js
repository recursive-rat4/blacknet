/*
 * Copyright (c) 2018-2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

$(document).ready(function () {

    const menu = $('.main-menu'), panel = $('.rightpanel'), apiVersion = "/api/v1", body = $("body");
    const hash = localStorage.hashIndex || 'overview';
    const dialogPassword = $('.dialog.password'),mask = $('.mask');
    let blockStack = [];

    menu.find('a[data-index="' + hash + '"]').parent().addClass('active');
    
    
    function getMnemoic(){
        
        return $.trim(dialogPassword.find('.mnemonic').val());
    }
    function staking_click(type) {

        return function () {
            mask.show();
            dialogPassword.show().find('.confirm').unbind().on('click', function () {

                let mnemonic = getMnemoic();

                //验证助记词
                if(!Blacknet.verifyMnemonic(mnemonic)){
                    Blacknet.message("Invalid mnemonic", "warning")
                    dialogPassword.find('.mnemonic').focus()
                    return
                }   
                type == 'isStaking' ? refreshStaking(mnemonic, type) : changeStaking(mnemonic, type);
            });
        }
    }

    async function postStaking(mnemonic, type, callback){

        let url = '/' + type;
        let formdata = new FormData();
        formdata.append('mnemonic', mnemonic);
        Blacknet.postV2(url, formdata, callback, function () {
            clearPassWordDialog();
            timeAlert('Invalid mnemonic');
        });
    }

    async function refreshStaking(mnemonic, type) {
        
        let stakingText = $('.is_staking');
        stakingText.text('loading');
        clearPassWordDialog();
        await Blacknet.wait(1000);
        postStaking(mnemonic, type, function(ret){
            localStorage.isStaking = ret;
            stakingText.text(ret);
        });
    }

    async function changeStaking(mnemonic, type) {

        postStaking(mnemonic, type, function(ret){
            
            let msg = ret == 'false' ? 'FAILED!' :'SUCCESS!';

            timeAlert(type + ' ' + msg);
            clearPassWordDialog();
            refreshStaking(mnemonic, 'isStaking');
        });
    }

    function menuSwitch() {

        const target = $(this), index = target.find('a').attr('data-index');

        target.addClass('active').siblings().removeClass('active');
        panel.find('.' + index).show().siblings().hide();

        localStorage.hashIndex = index;
        return false;
    }

    function sign() {
        let mnemonic = $('#sign_mnemonic').val();
        mnemonic = $.trim(mnemonic);
        let message = $('#sign_message').val();
        if(!Blacknet.verifyMnemonic(mnemonic)){
            Blacknet.message("Invalid mnemonic", "warning")
            $('#sign_mnemonic').focus()
            return
        }
        if(!Blacknet.verifyMessage(message)){
            Blacknet.message("Invalid message", "warning")
            $('#sign_message').focus()
            return
        }
        let data = new FormData();
        data.append('mnemonic', mnemonic);
        data.append('message', message);

        Blacknet.postV2('/signmessage', data, function(data){
            $('#sign_result').text(data).parent().removeClass("hidden")
        });
    }
    function verify() {
        let account = $('#verify_account').val();
        let signature = $('#verify_signature').val();
        let message = $('#verify_message').val();
        let url = "/verifymessage/" + account + "/" + signature + "/" + message + "/";
        if(!Blacknet.verifyAccount(account)) {
            Blacknet.message("Invalid account", "warning")
            $('#verify_account').focus()
            return 
        }
        if(!Blacknet.verifySign(signature)){
            Blacknet.message("Invalid signature", "warning")
            $('#verify_signature').focus()
            return
        }
        if(!Blacknet.verifyMessage(message)){
            Blacknet.message("Invalid message", "warning")
            $('#verify_message').focus()
            return
        }

        let data = new FormData();
        data.append('account', account);
        data.append('signature', signature);
        data.append('message', message);

        Blacknet.postV2('/verifymessage', data, function(data){
            $('#sign_result').text(data).parent().removeClass("hidden")
        });
    }
    function mnemonic_info() {
        let mnemonic = $('#mnemonic_info_mnemonic').val();
        mnemonic = $.trim(mnemonic);
        let formdata = new FormData();
        formdata.append('mnemonic', mnemonic);

        if(!Blacknet.verifyMnemonic(mnemonic)){
            Blacknet.message("Invalid mnemonic", "warning")
            $('#mnemonic_info_mnemonic').focus()
            return
        }
        Blacknet.postV2('/mnemonic/info', formdata, function (data) {
            let html = '';
            data.mnemonic = data.mnemonic.replace(/[a-z]/g, '*');

            html += 'mnemonic: ' + data.mnemonic;
            html += '\naddress: ' + data.address;
            html += '\npublicKey: ' + data.publicKey;
            $('#mnemonic_info_result').text(html).parent().removeClass("hidden")
        }, 'json');
    }

    function transfer_click(type) {
        return function () {
            switch (type) {
                case 'send': transfer(); break;
                case 'lease': lease(); break;
                case 'cancel_lease': cancel_lease(); break;
            }
        }
    }

    function input_mnemonic(fn){
        mask.show();
        dialogPassword.show().find('.confirm').unbind().on('click', function () {
            let mnemonic = getMnemoic();
            //验证助记词
            if(!Blacknet.verifyMnemonic(mnemonic)){
                Blacknet.message("Invalid mnemonic", "warning")
                dialogPassword.find('.mnemonic').focus()
                return
            }
            if(Object.prototype.toString.call(fn) === "[object Function]"){
                fn.call(this, mnemonic);
            }
        });
    }

    function transfer() {
        let to = $('#transfer_to').val();
        let amount = $('#transfer_amount').val();
        let message = $('#transfer_message').val();
        let encrypted = message && $('#transfer_encrypted').prop('checked') ? "1" : "";
        if(!Blacknet.verifyAccount(to)) {
            Blacknet.message("Invalid account", "warning")
            $('#transfer_to').focus()
            return 
        }
        if(!Blacknet.verifyAmount(amount)) {
            Blacknet.message("Invalid amount", "warning")
            $('#transfer_amount').focus()
            return 
        }
        input_mnemonic(function (mnemonic) {
            
            Blacknet.sendMoney(mnemonic, amount, to, message, encrypted, function (data) {
                $('#transfer_result').text(data).parent().removeClass("hidden")
                clearPassWordDialog();
            });
        })
    }

    function lease() {
        let to = $('#lease_to').val();
        let amount = $('#lease_amount').val();
        if(!Blacknet.verifyAccount(to)) {
            Blacknet.message("Invalid account", "warning")
            $('#lease_to').focus()
            return 
        }
        if(!Blacknet.verifyAmount(amount)) {
            Blacknet.message("Invalid amount", "warning")
            $('#lease_amount').focus()
            return 
        }
        input_mnemonic(function (mnemonic) {
            Blacknet.lease(mnemonic, 'lease', amount, to, 0, function (data) {
                $('#lease_result').text(data).parent().removeClass("hidden")
                clearPassWordDialog();
            });
        })
    }

    function cancel_lease(mnemonic) {

        let to = $('#cancel_lease_to').val();
        let amount = $('#cancel_lease_amount').val();
        let height = $('#cancel_lease_height').val();
        if(!Blacknet.verifyAccount(to)) {
            Blacknet.message("Invalid account", "warning")
            $('#cancel_lease_to').focus()
            return 
        }
        if(!Blacknet.verifyAmount(amount)) {
            Blacknet.message("Invalid amount", "warning")
            $('#cancel_lease_amount').focus()
            return 
        }
        input_mnemonic(function (mnemonic) {
            Blacknet.lease(mnemonic, 'cancellease', amount, to, height, function (data) {
                $('#cancel_lease_result').text(data).parent().removeClass("hidden")
                clearPassWordDialog();
            });
        })
    }

    function clearPassWordDialog() {
        mask.hide();
        dialogPassword.hide().find('.confirm').unbind();
        dialogPassword.find('.mnemonic').val('');
    }

    function timeAlert(msg, timeout) {
        setTimeout(function () {
            Blacknet.message(msg, "warning")
        }, timeout || 100);
    }

    function switchAccount() {

        localStorage.account = "";
        location.reload();
    }

    async function newAccount() {
        $('.account.dialog').hide();
        $('.newaccount.dialog').show();
        let url = '/account/generate';
        let mnemonicInfo = await Blacknet.getPromise(url);
        mnemonicInfo = JSON.parse(mnemonicInfo);
        $('#new_account_text').val(mnemonicInfo.address);
        $('#new_mnemonic').val(mnemonicInfo.mnemonic);
        window.isGenerated = true;
    }

    function newAccountNext() {

        let status = $('#confirm_mnemonic_warning').prop('checked');

        if (!status) {

            $('#confirm_mnemonic_warning_container label').css('color', 'red');
        } else {
            location.reload();
        }
        return false;
    }

    const block_request = function (message) {

        if (message.data) {
            let block = JSON.parse(message.data);

            blockStack.push(block);
        }
    };


    async function blockStackProcess() {

        let block;

        if (blockStack.length == 0) return;

        if (blockStack.length > 100) {

            blockStack = blockStack.slice(-35);
        }

        block = blockStack.shift();

        await Blacknet.renderBlock(block, block.height);
        await Blacknet.network();

        if (blockStack.length == 0) {
            Blacknet.refreshBalance();
            Blacknet.refreshTxConfirmations();
        }
    }

    setInterval(blockStackProcess, 100);

    function confirm_mnemonic_warning() {
        window.isGenerated = !this.checked;
    }

    async function addPeer() {
        let address = $('#ip_address').val(), port = $('#ip_port').val();

        if(!Blacknet.verifyNetworkAddress(address)) {
            Blacknet.message("Invalid address", "warning")
            $('#ip_address').focus()
            return 
        }

        let result = await Blacknet.getPromise(`/addpeer/${ip}/${port}/true`);

        await Blacknet.network();

        Blacknet.message(`${ip} ${result}`, "success")

        $('#ip_address').val('');
    }

    Blacknet.ready(function () {

        let blockNotify = new WebSocket("ws://" + location.host + "/api/v2/notify/block");
        let transactionNotify = new WebSocket("ws://" + location.host + "/api/v1/notify/transaction");
        
        blockNotify.onmessage = block_request;
        transactionNotify.onmessage = function(message){
            if (message.data) {
                let tx = JSON.parse(message.data);
    
                Blacknet.renderTransaction(tx, true);
                if(tx.time * 1000 > Date.now() - 1000*60){
                    Blacknet.newTransactionNotify(tx);
                }
            }
        };

        transactionNotify.onopen = function(){

            transactionNotify.send(localStorage.account);
        };
        
    });


    menu.on('click', 'li', menuSwitch);
    panel.find('.' + hash).show();

    body.on("click", "#stop_staking", staking_click('stopStaking'))
        .on("click", "#start_staking", staking_click('startStaking'))
        .on("click", "#refresh_staking", staking_click('isStaking'))
        .on("click", "#transfer", transfer_click('send'))
        .on("click", "#lease", transfer_click('lease'))
        .on("click", "#cancel_lease", transfer_click('cancel_lease'))
        .on("click", "#sign", sign)
        .on("click", "#verify", verify)
        .on("click", "#mnemonic_info", mnemonic_info)
        .on("click", "#add_peer_btn", addPeer)
        .on("click", "#switch_account", switchAccount)
        .on("click", "#new_account", newAccount)
        .on("input", "#confirm_mnemonic_warning", confirm_mnemonic_warning)
        .on("click", "#new_account_next_step", newAccountNext)
        .on("click", "tr.preview", function(){
            $(this).next().toggle();
            $(this).hide();
        })
        .on("click", ".tx-foot .show_more_txs", function(){
            $(this).hide();
            Blacknet.showMoreTxs();
        });


});