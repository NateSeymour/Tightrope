import production from './config.prod';
import development from './config.dev';
import base from './config.base';

export default {
    ...(process.env.NODE_ENV === 'production' ? production : development),
    ...base,
};