$(document).ready(function () {

    const menu = $('.main-menu'), panel = $('.rightpanel');
    const hash = localStorage.hashIndex || 'overview';



    menu.on('click', 'li', menuSwitch);
    menu.find('a[data-index="'+hash+'"]').parent().addClass('active');
   
    panel.find('.'+hash).show();



        
    function menuSwitch(){
        
        const target = $(this), index = target.find('a').attr('data-index');

        target.addClass('active').siblings().removeClass('active');
        panel.find('.'+index).show().siblings().hide();

        localStorage.hashIndex = index;
        return false;
    }
        
});