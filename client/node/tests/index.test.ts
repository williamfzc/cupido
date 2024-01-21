import { CupidoClient } from '../src';

describe('CupidClient', () => {
    it('should fetch root data successfully', async () => {
        const client = new CupidoClient();
        const rootData = await client.api_root();
        expect(rootData).toBeDefined();
    });

    it('should fetch size data successfully', async () => {
        const client = new CupidoClient();
        const sizeData = await client.api_size();
        expect(sizeData).toBeDefined();
    });
});
