


async function i18n(config) {

    let data = await $.getJSON('i18n/' + config.locale + '.json');
    let json = {};

    $(document).find('[data-i18n]').each(function () {

        let el = $(this), key = el.data('i18n');
        json[key] = '';
        
        if (data[key]) {
            el.text(data[key]);
        }
    });
}