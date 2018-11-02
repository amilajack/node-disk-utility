const addon = require('..');

describe('basic', () => {
    it('should get dir size', () => {
        console.log((addon.dirSize(__dirname)));
    });

    it('should fail if dir not given', () => {
        expect(() => {
            addon.dirSize();
        }).toThrow();
    });
});