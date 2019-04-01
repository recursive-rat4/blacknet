/*
 * Copyright (c) 2018-2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

void function(){

    const Blacknet = {};
    const menu = $('.main-menu'), panel = $('.rightpanel'), apiVersion = "/api/v1", body = $("body");;
    const hash = localStorage.hashIndex || 'overview';
    const dialogPassword = $('.dialog.password'), mask = $('.mask');
    const account = localStorage.account;
    const dialogAccount = $('.dialog.account');


    Blacknet.init = function(){

        if(account){

            mask.removeClass('init').hide();
            $('.dialog.account').hide();

            mask.on('click', function(){
                mask.hide();
                dialogPassword.hide();
            });
        }else{
            dialogAccount.find('.enter').unbind().on('click', function(){
                
                let account = dialogAccount.find('.account_text').val();

                if(/^blacknet[a-z0-9]{59}$/.test(account)){

                    localStorage.account = account;
                    location.reload();
                }else{
                    alert('Invalid Account');
                }
            });
        }
    };

    Blacknet.balance = async function(){

        let balance = $('.balance');

        $.getJSON(apiVersion + '/ledger/get/' + account + '/', function(data){
            balance.html( new BigNumber(data.balance).dividedBy(1e8) + ' BLN');
        }).fail(function(){
            balance.html('0.00000000 BLN');
        });
    };

    Blacknet.network = async function(){
        let network = $('.network');
        $.getJSON(apiVersion + '/ledger', function(data){
            
            network.find('.height').html(data.height);
            network.find('.supply').html((data.supply/1e8).toFixed(0));
            network.find('.accounts').html(data.accounts);
        });

        $.getJSON(apiVersion + '/nodeinfo', function(data){

            network.find('.connections').html(data.incoming);
        });
    };
    Blacknet.get = function(url, callback){

        return $.get(apiVersion + url, callback);
    };
    Blacknet.getPromise = function(url){

        return $.get(apiVersion + url);
    };
    Blacknet.post = function(url, callback, type){
        return $.post(apiVersion + url, {}, callback, type);
    };

    Blacknet.init();
    Blacknet.network();
    Blacknet.balance();
    window.Blacknet = Blacknet;
}();