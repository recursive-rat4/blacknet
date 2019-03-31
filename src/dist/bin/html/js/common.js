
void function(){

    const Blacknet = {};
    const menu = $('.main-menu'), panel = $('.rightpanel'), apiVersion = "/api/v1", body = $("body");;
    const hash = localStorage.hashIndex || 'overview';
    const dialogPassword = $('.dialog.password'), mask = $('.mask');
    const account = localStorage.account || 'blacknet1mm29uzgw40vl3mtaf3mepserc0vtmuapvmx5l92qxggvx0aqlnysp3v2hz';




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
            console.log(data.incoming);
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

    Blacknet.network();
    Blacknet.balance();
    window.Blacknet = Blacknet;
}();