/*
 * Copyright (c) 2018-2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

$(document).ready(function () {

    const menu = $('.main-menu'), panel = $('.rightpanel'), apiVersion = "/api/v1", body = $("body");;
    const hash = localStorage.hashIndex || 'overview';
    const dialogPassword = $('.dialog.password'), mask = $('.mask');


    menu.find('a[data-index="' + hash + '"]').parent().addClass('active');




    function staking_click(type) {

        return function () {
            mask.show();
            dialogPassword.show().find('.confirm').unbind().on('click', function () {

                let mnemonic = dialogPassword.find('.mnemonic').val();

                type == 'refresh_staking' ? refreshStaking(mnemonic) : post_staking(mnemonic, type);
            });
        }
    }


    async function post_staking(mnemonic, type) {

        let url = apiVersion + "/" + type + "Staking/" + mnemonic + "/";

        $.post(url, {}, function (ret) {

            let msg = ret == 'false' ? type.toUpperCase() + ' FAILED!' : type.toUpperCase() + ' SUCCESS!';
            clearPassWordDialog();
            refreshStaking(mnemonic);
            timeAlert(msg);
        }).fail(function () {
            clearPassWordDialog();
            timeAlert('Invalid mnemonic');
        });
    }

    async function refreshStaking(mnemonic) {

        let stakingText = $('.isStaking'), data;

        stakingText.text('loading');
        data = await Blacknet.postPromise('/isStaking/' + mnemonic);
        localStorage.isStaking = data;
        clearPassWordDialog();
        await Blacknet.wait(2000);
        $('.isStaking').text(data);
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
        let message = $('#sign_message').val();
        let url = "/signmessage/" + mnemonic + "/" + message + "/";

        Blacknet.post(url, function (data) {
            $('#sign_result').val(data);
        });
    }
    function verify() {
        let account = $('#verify_account').val();
        let signature = $('#verify_signature').val();
        let message = $('#verify_message').val();
        let url = "/verifymessage/" + account + "/" + signature + "/" + message + "/";

        Blacknet.get(url, function (data) {
            $('#verify_result').val(data);
        });
    }
    function mnemonic_info() {
        let mnemonic = $('#mnemonic_info_mnemonic').val();
        let url = "/mnemonic/info/" + mnemonic + "/";

        Blacknet.post(url, function (data) {
            let html = '';

            data.mnemonic = data.mnemonic.replace(/[a-z]/g, '*');

            html += 'mnemonic: ' + data.mnemonic;
            html += '<br>address: ' + data.address;
            html += '<br>publicKey: ' + data.publicKey;
            $('#mnemonic_info_result').html(html);
        }, 'json');
    }

    async function generate_new_account() {
        let url = '/account/generate';
        let blockData = await Blacknet.getPromise(url);
        blockData = JSON.parse(blockData);
        $('#new_account').val(blockData.address);
        $('#new_mnemonic').val(blockData.mnemonic);
        $('#new_pubkey').val(blockData.publicKey);
    }

    function transfer_click(type) {

        return function () {
            mask.show();
            dialogPassword.show().find('.confirm').unbind().on('click', function () {

                let mnemonic = dialogPassword.find('.mnemonic').val();
                switch (type) {
                    case 'send': transfer(mnemonic); break;
                    case 'lease': lease(mnemonic); break;
                    case 'cancel_lease': cancel_lease(mnemonic); break;
                }
            });
        }
    }

    function transfer(mnemonic) {

        let to = $('#transfer_to').val();
        let amount = $('#transfer_amount').val();
        let message = $('#transfer_message').val();
        let encrypted = message && $('#transfer_encrypted').prop('checked') ? "1" : "";

        Blacknet.sendMoney(mnemonic, amount, to, message, encrypted, function (data) {
            $('#transfer_result').val(data);
            clearPassWordDialog();
        });
    }

    function lease(mnemonic) {

        let to = $('#lease_to').val();
        let amount = $('#lease_amount').val();

        Blacknet.lease(mnemonic, 'lease', amount, to, 0, function (data) {
            $('#lease_result').val(data);
            clearPassWordDialog();
        });
    }

    function cancel_lease(mnemonic) {

        let to = $('#cancel_lease_to').val();
        let amount = $('#cancel_lease_amount').val();
        let height = $('#cancel_lease_height').val();

        Blacknet.lease(mnemonic, 'cancellease', amount, to, height, function (data) {
            $('#cancel_lease_result').val(data);
            clearPassWordDialog();
        });
    }

    function clearPassWordDialog() {
        mask.hide();
        dialogPassword.hide().find('.confirm').unbind();
        dialogPassword.find('.mnemonic').val('');
    }

    function timeAlert(msg, timeout) {
        setTimeout(function () {
            alert(msg);
        }, timeout || 100);
    }

    function switchAccount() {

        localStorage.account = "";
        location.reload();
    }

    async function append_block(hash, height) {

        let url = `/blockdb/get/${hash}`;
        let block = await Blacknet.getPromise(url, 'json');

        let tmpl = `<tr><td class="narrow height">${height}</td>
                    <td class="size narrow">${block.size}</td>
                    <td class="time narrow">${Blacknet.unix_to_local_time(block.time)}</td>
                    <td class="txns narrow">${block.transactions.length}</td>
                    <td class="generator">${block.generator}</td></tr>`;

        $(tmpl).prependTo("#block-list");

        let rowsCount = $('#block-list').find('tr').length;

        if (rowsCount > 15) {
            $('#block-list tr:last-child').remove();
        }
    }

    async function request_info(message = {}) {

        Blacknet.network();

        if (message.data) {
            append_block(message.data, Blacknet.height);
            Blacknet.height++;
        }
    }


    let ws = new WebSocket("ws://" + location.host + "/api/v1/notify/block");
    ws.onmessage = request_info;


    menu.on('click', 'li', menuSwitch);
    panel.find('.' + hash).show();

    body.on("click", "#stop_staking", staking_click('stop'))
        .on("click", "#start_staking", staking_click('start'))
        .on("click", "#refresh_staking", staking_click('refresh_staking'))
        .on("click", "#transfer", transfer_click('send'))
        .on("click", "#lease", transfer_click('lease'))
        .on("click", "#cancel_lease", transfer_click('cancel_lease'))
        .on("click", "#sign", sign)
        .on("click", "#verify", verify)
        .on("click", "#mnemonic_info", mnemonic_info)
        .on("click", "#generate_new_account", generate_new_account)
        .on("click", "#switch_account", switchAccount)

});