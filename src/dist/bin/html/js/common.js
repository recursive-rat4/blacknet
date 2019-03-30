
void function(){

    const Blacknet = {};
    const menu = $('.main-menu'), panel = $('.rightpanel'), apiVersion = "/api/v1", body = $("body");;
    const hash = localStorage.hashIndex || 'overview';
    const dialogPassword = $('.dialog.password'), mask = $('.mask');
    const account = localStorage.account || 'blacknet1mm29uzgw40vl3mtaf3mepserc0vtmuapvmx5l92qxggvx0aqlnysp3v2hz';




    Blacknet.balance = async function(){

        let balance = $('.balance');

        $.getJSON(apiVersion + '/ledger/get/' + account + '/', function(data){
            balance.html( (data.balance/1e8).toFixed(8) + ' BLN');
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
    };
    Blacknet.get = function(url, callback){

        return $.get(apiVersion + url, callback);
    };
    Blacknet.post = function(url, callback, type){
        return $.post(apiVersion + url, {}, callback, type);
    };

    Blacknet.network();
    Blacknet.balance();
    window.Blacknet = Blacknet;
}();