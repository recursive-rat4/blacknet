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

    menu.on('click', 'li', menuSwitch);
    body.on("click", "#start_staking", start_staking_click);
    panel.find('.'+hash).show();
        // .on("click", "#stop_staking", stop_staking)
        // .on("click", "#balance", balance)
        // .on("click", "#transfer", transfer)
        // .on("click", "#sign", sign)
        // .on("click", "#verify", verify)
        // .on("click", "#mnemonic_info", mnemonic_info)
        // .on("click", "#generate_new_account", generate_new_account)
        // .on("click", "#display_api_json_result", function (event) {
        //     let el = event.target;
        //     display_api_json_result(el.dataset.type);
        // });
    
});