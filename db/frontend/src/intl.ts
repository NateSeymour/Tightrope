export const locale = [navigator.language] || navigator.languages;

export const localTimeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;

export const dateTimeFormat = new Intl.DateTimeFormat(locale, {
    month: 'short',
    day: 'numeric',
    timeZone: localTimeZone,
});

export const dateTimeFormatLong = new Intl.DateTimeFormat(locale, {
    month: 'long',
    day: 'numeric',
    year: 'numeric',
    timeZone: localTimeZone,
});

export const dayFormat = new Intl.DateTimeFormat(locale, {
    weekday: 'long',
    timeZone: localTimeZone,
});

export const monthYearFormat = new Intl.DateTimeFormat(locale, {
    month: 'long',
    year: 'numeric',
    timeZone: localTimeZone,
});