import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  useUsers,
  useRoles,
  useCreateUser,
  useUpdateUser,
  useChangePassword,
} from '@/hooks/useApi';
import { toast } from 'sonner';
import { Plus, Pencil, KeyRound } from 'lucide-react';
import type { User } from '@/types/user';

export function UsersPage() {
  const { data: users, isLoading } = useUsers();
  const { data: roles } = useRoles();
  const createUser = useCreateUser();
  const updateUser = useUpdateUser();
  const changePassword = useChangePassword();

  const [createOpen, setCreateOpen] = useState(false);
  const [editUser, setEditUser] = useState<User | null>(null);
  const [pwUser, setPwUser] = useState<User | null>(null);

  const [createForm, setCreateForm] = useState({
    email: '',
    password: '',
    full_name: '',
    role_id: '',
  });
  const [editForm, setEditForm] = useState({
    email: '',
    full_name: '',
    role_id: '',
    is_active: true,
  });
  const [newPassword, setNewPassword] = useState('');

  function openCreate() {
    setCreateForm({ email: '', password: '', full_name: '', role_id: '' });
    setCreateOpen(true);
  }

  function openEdit(user: User) {
    setEditUser(user);
    setEditForm({
      email: user.email,
      full_name: user.full_name,
      role_id: user.role_id,
      is_active: user.is_active,
    });
  }

  function handleCreate() {
    createUser.mutate(createForm, {
      onSuccess: () => {
        toast.success('User created');
        setCreateOpen(false);
      },
      onError: () => toast.error('Failed to create user'),
    });
  }

  function handleUpdate() {
    if (!editUser) return;
    updateUser.mutate(
      { id: editUser.id, data: editForm },
      {
        onSuccess: () => {
          toast.success('User updated');
          setEditUser(null);
        },
        onError: () => toast.error('Failed to update user'),
      },
    );
  }

  function handleToggleActive(user: User) {
    updateUser.mutate(
      {
        id: user.id,
        data: {
          email: user.email,
          full_name: user.full_name,
          role_id: user.role_id,
          is_active: !user.is_active,
        },
      },
      {
        onSuccess: () =>
          toast.success(user.is_active ? 'User deactivated' : 'User activated'),
        onError: () => toast.error('Failed to update status'),
      },
    );
  }

  function handleChangePassword() {
    if (!pwUser) return;
    changePassword.mutate(
      { id: pwUser.id, data: { new_password: newPassword } },
      {
        onSuccess: () => {
          toast.success('Password changed');
          setPwUser(null);
          setNewPassword('');
        },
        onError: () => toast.error('Failed to change password'),
      },
    );
  }

  const list = users ?? [];

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">Users</h2>
          <p className="text-sm text-muted-foreground">Manage user accounts and roles</p>
        </div>
        <Button size="sm" onClick={openCreate}>
          <Plus className="mr-1 h-4 w-4" /> Add User
        </Button>
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 3 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : list.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Name</TableHead>
                  <TableHead className="hidden sm:table-cell">Email</TableHead>
                  <TableHead>Role</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead className="w-28">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((u) => (
                  <TableRow key={u.id}>
                    <TableCell className="font-medium">{u.full_name}</TableCell>
                    <TableCell className="hidden sm:table-cell">{u.email}</TableCell>
                    <TableCell>
                      <Badge variant="outline" className="capitalize">
                        {u.role_name}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <Badge
                        variant={u.is_active ? 'default' : 'secondary'}
                        className="cursor-pointer"
                        onClick={() => handleToggleActive(u)}
                      >
                        {u.is_active ? 'Active' : 'Inactive'}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button
                          variant="ghost"
                          size="icon"
                          className="h-8 w-8"
                          onClick={() => openEdit(u)}
                          title="Edit user"
                        >
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          className="h-8 w-8"
                          onClick={() => { setPwUser(u); setNewPassword(''); }}
                          title="Change password"
                        >
                          <KeyRound className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">No users yet.</p>
          )}
        </CardContent>
      </Card>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>New User</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>Full Name</Label>
              <Input
                value={createForm.full_name}
                onChange={(e) => setCreateForm({ ...createForm, full_name: e.target.value })}
              />
            </div>
            <div>
              <Label>Email</Label>
              <Input
                type="email"
                value={createForm.email}
                onChange={(e) => setCreateForm({ ...createForm, email: e.target.value })}
              />
            </div>
            <div>
              <Label>Password</Label>
              <Input
                type="password"
                value={createForm.password}
                onChange={(e) => setCreateForm({ ...createForm, password: e.target.value })}
              />
            </div>
            <div>
              <Label>Role</Label>
              <Select
                value={createForm.role_id}
                onValueChange={(v) => setCreateForm({ ...createForm, role_id: v })}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select role" />
                </SelectTrigger>
                <SelectContent>
                  {(roles ?? []).map((r) => (
                    <SelectItem key={r.id} value={r.id}>
                      {r.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button
              onClick={handleCreate}
              className="w-full"
              disabled={createUser.isPending || !createForm.email || !createForm.password || !createForm.role_id}
            >
              Create User
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={!!editUser} onOpenChange={(open) => !open && setEditUser(null)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Edit User</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>Full Name</Label>
              <Input
                value={editForm.full_name}
                onChange={(e) => setEditForm({ ...editForm, full_name: e.target.value })}
              />
            </div>
            <div>
              <Label>Email</Label>
              <Input
                type="email"
                value={editForm.email}
                onChange={(e) => setEditForm({ ...editForm, email: e.target.value })}
              />
            </div>
            <div>
              <Label>Role</Label>
              <Select
                value={editForm.role_id}
                onValueChange={(v) => setEditForm({ ...editForm, role_id: v })}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select role" />
                </SelectTrigger>
                <SelectContent>
                  {(roles ?? []).map((r) => (
                    <SelectItem key={r.id} value={r.id}>
                      {r.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button
              onClick={handleUpdate}
              className="w-full"
              disabled={updateUser.isPending}
            >
              Update User
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Change Password Dialog */}
      <Dialog open={!!pwUser} onOpenChange={(open) => !open && setPwUser(null)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Change Password — {pwUser?.full_name}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>New Password</Label>
              <Input
                type="password"
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
              />
            </div>
            <Button
              onClick={handleChangePassword}
              className="w-full"
              disabled={changePassword.isPending || !newPassword}
            >
              Change Password
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
