/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {}

    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};
*/

class Solution {
public:
    void breathSearch(Node* root) {
        if (root == nullptr) {
            return;
        }

        queue<Node*> nodeQueue;

        if (root->left != nullptr) {
            nodeQueue.push(root->left);
        }
        if (root->right != nullptr) {
            nodeQueue.push(root->right);
        }

        while (!nodeQueue.empty()) {
            int queueSize = nodeQueue.size();

            for (int i = 0; i < queueSize; i++) {
                Node* currentNode = nodeQueue.front();

                nodeQueue.pop();

                if (i < queueSize - 1) {
                    currentNode->next = nodeQueue.front();     
                }

                if (currentNode->left != nullptr) {
                    nodeQueue.push(currentNode->left);
                }

                if (currentNode->right != nullptr) {
                    nodeQueue.push(currentNode->right);
                }
            }
        }

    }
    Node* connect(Node* root) {
        breathSearch(root);

        return root;
    }
};