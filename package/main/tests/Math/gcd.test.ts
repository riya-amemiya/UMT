import { gcd } from '../../module/Math/gcd';
test('{gcd}', () => {
    expect(gcd(1, 1)).toBe(1);
    expect(gcd(1, 2)).toBe(1);
    expect(gcd(2, 1)).toBe(1);
    expect(gcd(2, 2)).toBe(2);
    expect(gcd(2, 3)).toBe(1);
    expect( gcd( 2, 4 ) ).toBe( 2 );
    expect( gcd( 4, 2 ) ).toBe( 2 );
    expect( gcd( 2, 6 ) ).toBe( 2 );
    expect( gcd( 6, 2 ) ).toBe( 2 );
    expect( gcd( 2, 8 ) ).toBe( 2 );
    expect( gcd( 8, 2 ) ).toBe( 2 );
    expect( gcd( 2, 10 ) ).toBe( 2 );
    expect( gcd( 10, 2 ) ).toBe( 2 );

});
