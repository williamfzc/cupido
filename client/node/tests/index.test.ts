import { CupidClient } from '../src';

describe('CupidClient', () => {
    it('should fetch root data successfully', async () => {
        const client = new CupidClient();
        const rootData = await client.api_root();
        expect(rootData).toBeDefined();
    });

    it('should fetch size data successfully', async () => {
        const client = new CupidClient();
        const sizeData = await client.api_size();
        expect(sizeData).toBeDefined();
    });
});
