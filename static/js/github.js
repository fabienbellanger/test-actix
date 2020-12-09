$(function () {
    const now = moment();

    $('.datetime').each(function () {
        $(this).html(moment($(this).text().trim()).format('YYYY-MM-DD HH:mm'));
    });

    $('.datetime-human').each(function () {
        const datetime = moment($(this).text().trim());
        const duration2 = moment.duration(datetime.diff(now)).humanize(true);
        $(this).html(duration2);
    });

    $('#releases').DataTable({
        'pageLength': 25,
        'order': [[3, 'desc']],
    });

    const expiredAt = moment($("#cacheExpiredAt").text().trim());
    const duration = moment.duration(expiredAt.diff(now)).humanize(true);
    $('#cacheExpiredAt').html(duration);
});
