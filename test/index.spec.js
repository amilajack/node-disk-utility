const addon = require('..');

describe('basic', () => {
    it('should get dir size', () => {
        console.log((addon.dirSize()));
    })
})