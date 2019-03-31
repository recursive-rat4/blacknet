$(document).ready(function () {

    const menu = $('.main-menu'), panel = $('.rightpanel'), apiVersion = "/api/v1", body = $("body");;
    const hash = localStorage.hashIndex || 'overview';
    const dialogPassword = $('.dialog.password'), mask = $('.mask');

    const account = localStorage.account || 'blacknet1mm29uzgw40vl3mtaf3mepserc0vtmuapvmx5l92qxggvx0aqlnysp3v2hz';
    
    menu.find('a[data-index="'+hash+'"]').parent().addClass('active');
   
    


    function start_staking_click() {

        mask.show();
        dialogPassword.show();

        dialogPassword.find('.confirm').unbind().on('click', function(){
            start_staking(dialogPassword.find('.mnemonic').val());
        });
    }

    function start_staking(mnemonic){

        let url = apiVersion + "/staker/start/" + mnemonic + "/";

        $.post(url, {}, function(){
            hidePasswordDialog();
        }).fail(function(){
            hidePasswordDialog();
            alert('Invalid mnemonic');
        });
    }

    function hidePasswordDialog(){
         mask.hide();
         dialogPassword.hide();
     }

        
    function menuSwitch(){
        
        const target = $(this), index = target.find('a').attr('data-index');

        target.addClass('active').siblings().removeClass('active');
        panel.find('.'+index).show().siblings().hide();

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

        Blacknet.post( url, function (data) {
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

    function transfer_click(){
        mask.show();
        dialogPassword.show().find('.confirm').unbind().on('click', function(){
            transfer(dialogPassword.find('.mnemonic').val());
        });
    }

    function transfer(mnemonic) {
        
        let fee = 100000, amount, to, message, encrypted, amountText, url;

        to = $('#transfer_to').val();
        amount = $('#transfer_amount').val();
        message = $('#transfer_message').val();
        encrypted = message && $('#transfer_encrypted').prop('checked') ? "1" : "";

        amountText = new BigNumber(amount).toFixed(8);
        amount = new BigNumber(amount).times(1e8);

        url = "/transfer/" + mnemonic + "/" + fee + "/" + amount + "/" + to + "/" + message + "/" + encrypted + "/";

        if (confirm('Are you sure you want to send?\n\n' + amountText + ' BLN to \n' + to + '\n\n0.001 BLN added as transaction fee?')) {

            Blacknet.post(url, function (data) {
                $('#transfer_result').val(data);
            });
        }
    }

    mask.on('click', function(){
        hidePasswordDialog();
    });
    
    menu.on('click', 'li', menuSwitch);
    panel.find('.'+hash).show();

        // .on("click", "#stop_staking", stop_staking)
        // .on("click", "#balance", balance)
    body.on("click", "#start_staking", start_staking_click)
        .on("click", "#transfer", transfer_click)
        .on("click", "#sign", sign)
        .on("click", "#verify", verify)
        .on("click", "#mnemonic_info", mnemonic_info)
        .on("click", "#generate_new_account", generate_new_account)
        // .on("click", "#display_api_json_result", function (event) {
        //     let el = event.target;
        //     display_api_json_result(el.dataset.type);
        // });
    
});