const addon = require('..');

describe('basic', () => {
  it('should get dir size', () => {
    expect(typeof addon.dirSize(__dirname)).toEqual('string');
  });

  it('should fail if dir not given', () => {
    expect(() => {
      addon.dirSize();
    }).toThrow();
  });
});
