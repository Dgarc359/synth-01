import matplotlib.pyplot as plt
import math
import json

def generate_sin_wave():
    sample_rate = 44_100
    buf_size = 2048
    buf = [0.0] * buf_size 
    freqs = [ 440, 560 ]
    waves = []
    volume = 1

    for freq in freqs:
        wave = []
        for i, x in enumerate(buf):
            phase_angle = math.tau * freq * (i / sample_rate)
            wave.append(math.sin(phase_angle))
        
        waves.append(wave)
    
    print(waves)
    
    for i, x in enumerate(buf):
        buf[i] = sum(wave[i] for wave in waves) * volume

    
    for wave, freq in zip(waves, freqs):
        plt.plot(wave[:1000], linestyle='--', label=f"{freq} Hz")

    plt.plot(buf[:1000], linestyle='-')
    plt.xlabel("Index")
    plt.ylabel("Value")
    plt.title("Plot of a List")

    # Show the plot
    plt.show()


def read_out_files_01():
    with open('./out.txt') as f:
        with open('./out_buf.txt') as f2:
            f2 = f2.read()
            file = f.read()

            nums = [float(x) for x in file.split(",")]
            nums2 = [float(x) for x in f2.split(",")]


            plt.plot(nums2, linestyle='-')
            plt.plot(nums, linestyle='--')
            plt.xlabel("Index")
            plt.ylabel("Value")
            plt.title("Plot of a List")

            # Show the plot
            plt.show()



import time
import subprocess
import select

def main():
    figure = plt.figure()
    filename = "./out.txt"

    f = subprocess.Popen(['tail','-F',filename],\
            stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    p = select.poll()
    p.register(f.stdout)

    while True:
        if p.poll(1):
            read_string = f.stdout.readline().decode()
            if "UNALTERED_AUDIO_BUFFER" in read_string:
                print(f.stdout.readline())
        time.sleep(1)

    pass



if __name__ == "__main__":
    main()

 