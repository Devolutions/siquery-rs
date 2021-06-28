Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_video table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_video = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_video' }
				$wmi_video | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_video = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_video' }
				$schema = Get-Content $wmi_video.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_video = Get-Siq 'wmi_video' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_video' }).FullName -Raw

				$json = $wmi_video | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
