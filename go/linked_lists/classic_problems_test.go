package linkedlists

import (
	"testing"
)

func TestReverseLinkedList(t *testing.T){
	list := Constructor()
	list.AddAtHead(2)
	list.AddAtHead(1)

	reversed := reverseList(list.Head)
	if reversed.Val != 2 {
		t.Fail()
	}
}

func TestRemoveValFromList(t *testing.T) {
	list := Constructor()
	list.AddAtHead(5)
	list.AddAtHead(3)
	list.AddAtHead(3)
	list.AddAtHead(1)
	newHead := removeElements(list.Head, 3)
	if newHead.Next.Val == 3 {
		t.Fail()
	}
}
func TestRemoveValFromListMultipleLocations(t *testing.T) {
	list := Constructor()
	list.AddAtHead(6)
	list.AddAtHead(5)
	list.AddAtHead(4)
	list.AddAtHead(3)
	list.AddAtHead(6)
	list.AddAtHead(2)
	list.AddAtHead(1)
	newHead := removeElements(list.Head, 6)

	for newHead != nil {
		if newHead.Val == 6 {
			t.Fail()
		}
		newHead = newHead.Next
	}
}

func TestRemoveValFromListAllItems(t *testing.T) {
	list := Constructor()
	list.AddAtHead(6)
	list.AddAtHead(6)
	list.AddAtHead(6)
	list.AddAtHead(6)

	newHead := removeElements(list.Head, 6)

	for newHead != nil {
		if newHead.Val == 6 {
			t.Fail()
		}
		newHead = newHead.Next
	}
}

func TestOddEven(t *testing.T) {
	list := Constructor()
	list.AddAtHead(8)
	list.AddAtHead(7)
	list.AddAtHead(6)
	list.AddAtHead(5)
	list.AddAtHead(4)
	list.AddAtHead(3)
	list.AddAtHead(2)
	list.AddAtHead(1)
	newHead := oddEvenList(list.Head)

	for newHead != nil {
		if newHead.Val == 6 {
			t.Fail()
		}
		newHead = newHead.Next
	}
}


func TestIsPalindrome(t *testing.T) {
	list := Constructor()
	list.AddAtHead(1)
	list.AddAtHead(2)
	list.AddAtHead(3)
	list.AddAtHead(4)
	list.AddAtHead(4)
	list.AddAtHead(3)
	list.AddAtHead(2)
	list.AddAtHead(1)
	listIsPalindrome := isPalindrome(list.Head)

	if !listIsPalindrome {
			t.Fail()
	}
}

func TestIsNotPalindrome(t *testing.T) {
	list := Constructor()
	list.AddAtHead(1)
	list.AddAtHead(2)
	list.AddAtHead(3)
	list.AddAtHead(5)
	list.AddAtHead(4)
	list.AddAtHead(3)
	list.AddAtHead(2)
	list.AddAtHead(1)
	listIsPalindrome := isPalindrome(list.Head)

	if listIsPalindrome {
			t.Fail()
	}
}

