import { Component, Input, OnInit } from '@angular/core';
import { AuthService } from 'src/app/services/auth.service';
import LogonHistory from 'src/app/types/logon-history';

@Component({
  selector: 'osgp-com-logon-history-list',
  templateUrl: './logon-history-list.component.html',
  styleUrls: ['./logon-history-list.component.less'],
})
export class LogonHistoryListComponent implements OnInit {

  public history: LogonHistory[] = [];

  @Input()
  truncate: boolean = false

  constructor(
    public authService: AuthService,
  ) {
  }

  ngOnInit(): void {
    this.loadHistory();
  }

  public loadHistory() {
    this.authService.loadHistory()
      .then(history => this.applyHistory(history));
  }

  private applyHistory(history: LogonHistory[]) {
    if (this.truncate) {
      history = history.slice(0, 3);
    }

    this.history = history;
  }
}
