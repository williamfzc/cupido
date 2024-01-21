import axios, { AxiosResponse } from 'axios';

enum Endpoints {
    ROOT = "/",
    SIZE = "/size"
}

export class CupidClient {
    private apiBaseUrl: string;

    constructor(apiBaseUrl: string = 'http://127.0.0.1:9410') {
        this.apiBaseUrl = apiBaseUrl;
    }

    private async getData(endpoint: Endpoints): Promise<any> {
        try {
            const response: AxiosResponse<any> = await axios.get<any>(`${this.apiBaseUrl}${endpoint}`);
            return response.data;
        } catch (error: any) {
            console.error('Error:', error.message);
            throw error;
        }
    }

    async api_root(): Promise<any> {
        return this.getData(Endpoints.ROOT);
    }

    async api_size(): Promise<any> {
        return this.getData(Endpoints.SIZE);
    }
}
