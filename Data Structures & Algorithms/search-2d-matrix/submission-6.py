class Solution:
    def searchMatrix(self, matrix: list[list[int]], target: int) -> bool:
        Top_Bound = 0
        Bottom_Bound = len(matrix)-1
        Y_Center_Bound = Bottom_Bound // 2
        while Bottom_Bound >= Top_Bound:
            Right_Bound = len(matrix[0])-1
            Left_Bound = 0
            X_Center_Bound = Right_Bound // 2 
            qcen = matrix[Y_Center_Bound][Left_Bound]
            if matrix[Top_Bound][Left_Bound] <= target and matrix[Top_Bound][Right_Bound] >= target:
                while Right_Bound >= Left_Bound: # ???
                    if matrix[Top_Bound][X_Center_Bound] == target:
                        return True
                    elif matrix[Top_Bound][X_Center_Bound] > target:
                        Right_Bound = X_Center_Bound - 1
                        X_Center_Bound = Right_Bound // 2
                    elif matrix[Top_Bound][X_Center_Bound] < target:
                        Left_Bound = X_Center_Bound + 1
                        X_Center_Bound = (( Right_Bound - Left_Bound) // 2 )+ Left_Bound
            Top_Bound += 1

    
        return False