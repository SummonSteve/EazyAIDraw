import { SamplingMethods } from "./state";

export enum NovelAISamplingMethods {
    Plms = "plms",
    Ddim = "ddim",
    KEuler = "k_euler",
    KEulerAncestral = "k_euler_ancestral",
    KHuen = "k_heun",
    KDpm2 = "k_dpm_2",
    KDpm2Ancestral = "k_dpm_2_ancestral",
    KLms = "k_lms",
}

export class NovelAIDrawCall {
    public height: number;
    public width: number;
    public n_samples: number;
    public prompt: string;
    public sampler: NovelAISamplingMethods | SamplingMethods;
    public scale: number;
    public seed: number;
    public steps: number;
    public uc: string;
    public uc_preset: number;
    constructor(hight: number, width: number, n_samples: number, prompt: string, sampler: SamplingMethods, scale: number, seed: number, steps: number, uc: string, uc_preset: number) {
        this.height = hight;
        this.width = width;
        this.n_samples = n_samples;
        this.prompt = prompt;
        this.sampler = sampler;
        this.scale = scale;
        this.seed = seed;
        this.steps = steps;
        this.uc = uc;
        this.uc_preset = uc_preset;
        switch (sampler) {
            case SamplingMethods.ddim:
                this.sampler = NovelAISamplingMethods.Ddim;
                break;
            case SamplingMethods.plms:
                this.sampler = NovelAISamplingMethods.Plms;
                break;
            case SamplingMethods.euler:
                this.sampler = NovelAISamplingMethods.KEuler;
                break;
            case SamplingMethods.euler_a:
                this.sampler = NovelAISamplingMethods.KEulerAncestral;
                break;
            case SamplingMethods.heun:
                this.sampler = NovelAISamplingMethods.KHuen;
                break;
            case SamplingMethods.dpm2:
                this.sampler = NovelAISamplingMethods.KDpm2;
                break;
            case SamplingMethods.dpm2a:
                this.sampler = NovelAISamplingMethods.KDpm2Ancestral;
                break;
            case SamplingMethods.lms:
                this.sampler = NovelAISamplingMethods.KLms;
                break;
            default:
                this.sampler = NovelAISamplingMethods.KEuler;
        }
    }

    public into_json(): string {
        let json = {
            "draw_call": {
                "novel_ai": {
                    "height": Number(this.height),
                    "width": Number(this.width),
                    "n_samples": Number(this.n_samples),
                    "prompt": this.prompt,
                    "sampler": this.sampler,
                    "scale": Number(this.scale),
                    "seed": Number(this.seed),
                    "id": Number(this.seed),
                    "steps": Number(this.steps),
                    "uc": this.uc,
                    "uc_preset": Number(this.uc_preset),
                    
                }
            }
        }
        return JSON.stringify(json);
    }
}