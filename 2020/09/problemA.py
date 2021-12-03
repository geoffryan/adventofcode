import sys
import numpy as np

def checkForSum(nums, target):

    sums = nums[:, None] + nums[None, :]
    idx = np.arange(len(nums))

    oksums = sums[idx[:, None] < idx[None, :]]

    return (target == oksums).any()
    """

    for i in range(len(nums)):
        for j in range(i+1, len(nums)):
            if target == nums[i] + nums[j]:
                return True

    return False
    """


def checkSignal(nums, N):

    for idx in range(N, len(nums)):
        ia = idx - N
        ib = idx

        if not checkForSum(nums[ia:ib], nums[idx]):
            print("Not a sum!")
            return nums[idx]

    return None


if __name__ == "__main__":

    filename = sys.argv[1]
    N = int(sys.argv[2])

    nums = []

    with open(filename, "r") as f:
        for line in f:
            nums.append(int(line.strip()))

    nums = np.array(nums)

    badNum = checkSignal(nums, N)

    print(badNum)
