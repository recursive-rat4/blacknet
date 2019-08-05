


Blacknet.template = {


    transaction: async function (tx, account) {

        let amount = tx.data.amount, tmpl, txfee, type, status, txaccount = tx.from;

        type = Blacknet.getTxType(tx);
        txfee = Blacknet.getFormatBalance(tx.fee);

        if (tx.type == 254) {
            amount = tx.fee;
            txfee = '';
        }
        if (tx.type == 0 && tx.from == account) {
            txaccount = tx.data.to;
        }

        if (tx.time * 1000 < Date.now() - 1000 * 10) {
            status = 'Confirmed';
        } else {
            status = await Blacknet.getStatusText(tx.height, tx.hash);
        }
        
        let txText = type, linkText = '';


        if(tx.type == 0){
            let text = account == tx.from ? "Sent to" : 'Received from';
            txText = `<a target="_blank" href="https://www.blnscan.io/${tx.hash.toLowerCase()}">${text}</a>`
        }

        if(tx.type == 2 || tx.type == 3){
            txText = `<a target="_blank" href="https://www.blnscan.io/${tx.hash.toLowerCase()}">${type} ${account == tx.from ? "to" : 'from'}</a>`;
        }

        if(tx.type < 254 && tx.type != 0 && tx.type != 2 && tx.type != 3){
            txText = `<a target="_blank" href="https://www.blnscan.io/${tx.hash.toLowerCase()}">${type}</a>`;
        }

        if(tx.type == 0 || tx.type == 2 || tx.type == 3){

            if(account == tx.from){
                linkText = `<a target="_blank" href="https://www.blnscan.io/${tx.data.to}">${tx.data.to}</a>`;
            }else{
                linkText = `<a target="_blank" href="https://www.blnscan.io/${tx.from}">${tx.from}</a>`;
            }
        }

        if(tx.type != 0 && tx.type != 2 && tx.type != 3){

            if(tx.from != 'genesis'){
                linkText = `<a href="/${tx.from}">${tx.from}</a>`;
            }else{
                linkText = `<a href="/${tx.to}">${tx.to}</a>`;
            }
        }

        amount = Blacknet.getFormatBalance(amount);

        tmpl =
            `<tr class="preview txhash${tx.hash}" data-hash="${tx.hash}"  data-height="${tx.height}">
                <td class="narrow">${Blacknet.unix_to_local_time(tx.time)}</td>
                <td class="narrow">${txText}</td>
                <td class="left">${linkText}</td>
                <td class="right"><span class="strong">${amount}</span></td>
                <td class="left status" data-height="${tx.height}">${status}</td>
            </tr>`;


        let node = $(tmpl), msgNode = node.find('.message'), message;

        if (tx.type == 0) {
            if (tx.data.message.type == 0) {
                message = tx.data.message.message;
            } else if (tx.data.message.type == 1) {
                message = "Encrypted message";
            } else {
                message = "Non-standard message";
            }
            msgNode.text(message);
        }

        if (!message) node.find('.msg_text').hide();
        if (tx.type == 254) {
            node.find('.sign_text,.to,.fee').hide();
        }
        return node;
    },

    peer: function (peer) {

        let tmpl =
            `<tr>
                <td>${peer.peerId}</td>
                <td class="right">${peer.remoteAddress}</td>
                <td>${peer.agent}</td>
                <td class="right">${peer.ping} ms</td>
                <td class="narrow">${peer.timeOffset} s</td>
                <td class="narrow">${(peer.totalBytesRead/1048576).toFixed(2)} MiB</td>
                <td class="narrow">${(peer.totalBytesWritten/1048576).toFixed(2)} MiB</td>
            </tr>`;

        $(tmpl).appendTo("#peer-list");
    },

    block: async function (blockListEl, block, height, prepend = true) {

        let op = prepend ? 'prependTo' : 'appendTo';

        let tmpl = `<tr><td class="narrow height">
                        <a target="_blank" href="https://www.blnscan.io/${height}">${height}</a>
                    </td>
                    <td class="size narrow">${block.size}</td>
                    <td class="time narrow">${Blacknet.unix_to_local_time(block.time)}</td>
                    <td class="txns narrow">${block.txns}</td>
                    <td class="generator">
                        <a target="_blank" href="https://www.blnscan.io/${block.generator}">${block.generator}</a>
                    </td></tr>`;


        $(tmpl)[op](blockListEl);

        let rowsCount = blockListEl[0].childNodes.length;

        if (rowsCount > 36) {
            blockListEl.find('tr:last-child').remove();
        }
    },

    message: async function (msg, type) {
        var icon
        switch (type) {
            case "success":
                icon = '<i class="fa fa-check-circle"></i>'
                break;
            case "error":
                icon = '<i class="fa fa-times-circle"></i>';
                break;
            case "warning":
                icon = '<i class="fa fa-info-circle"></i>';
                break;
            default:
                icon = ''
                break;
        }
        var messageText = `<div class="blacknet-message-notice">
            <div class="blacknet-message-notice-content">${icon}${msg}
            </div>
        </div>`;
        var $msg = $(messageText)
        $(".blacknet-message").append($msg)
        setTimeout(function () {
            $msg.remove()
        }, 2000)
       
    }

};